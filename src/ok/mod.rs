use crate::{config, http, utils};
use anyhow::anyhow;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use log::{error, info};
use reqwest::{
    StatusCode,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
pub mod request;
pub mod response;

const API_BASE_URL: &str = "https://www.okx.com";
const METHOD_GET: &str = "GET";
const METHOD_POST: &str = "POST";

async fn sign(
    timestamp: &str,
    method: &str,
    request_path: &str,
    body: &str,
) -> anyhow::Result<String> {
    // OK-ACCESS-SIGN的请求头是对timestamp + method + requestPath + body字符串（+表示字符串连接），以及SecretKey，使用HMAC SHA256方法加密，通过Base-64编码输出而得到的。
    // 如：sign=CryptoJS.enc.Base64.stringify(CryptoJS.HmacSHA256(timestamp + 'GET' + '/api/v5/account/balance?ccy=BTC', SecretKey))
    // 其中，timestamp的值与OK-ACCESS-TIMESTAMP请求头相同，为ISO格式，如2020-12-08T09:08:57.715Z。
    // method是请求方法，字母全部大写：GET/POST。
    // requestPath是请求接口路径。如：/api/v5/account/balance
    // body是指请求主体的字符串，如果请求没有主体（通常为GET请求）则body可省略。如：{"instId":"BTC-USDT","lever":"5","mgnMode":"isolated"}
    let content = format!("{}{}{}{}", timestamp, method, request_path, body);
    let result = utils::secret::hmac_sha256(
        config::config()?.okx.secret_key.as_bytes(),
        content.as_bytes(),
    )
    .await?;
    Ok(BASE64_STANDARD.encode(result))
}

async fn okx_headers(timestamp: &str, sign: &str) -> anyhow::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "OK-ACCESS-KEY",
        HeaderValue::from_str(&config::config()?.okx.api_key).unwrap(),
    );
    headers.insert("OK-ACCESS-SIGN", HeaderValue::from_str(sign).unwrap());
    headers.insert(
        "OK-ACCESS-TIMESTAMP",
        HeaderValue::from_str(timestamp).unwrap(),
    );
    headers.insert(
        "OK-ACCESS-PASSPHRASE",
        HeaderValue::from_str(&config::config()?.okx.passphrase).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}

pub async fn get_swap_positions() -> anyhow::Result<Vec<response::PositionResponse>> {
    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    let url_str = "/api/v5/account/positions";
    let params = "?instType=SWAP";
    let header_map = okx_headers(
        &timestamp,
        &sign(&timestamp, METHOD_GET, url_str, params).await.unwrap(),
    )
    .await.map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;
    let url = format!("{}{}{}", API_BASE_URL, url_str, params);

    let resp = http::client()
        .await?
        .get(&url)
        .headers(header_map)
        .send()
        .await.map_err(|err| {
            error!("{}", err);
            anyhow!(err)
        })?;

    if resp.status() != StatusCode::OK {
        return Err(anyhow!("http error. {}", resp.status().as_u16()));
    }

    let text = resp.text().await.map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;

    info!("[{}] {} => {}", METHOD_GET, url, text);
    
    let response: response::Response<Vec<response::PositionResponse>> = serde_json::from_str(&text).map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;


    if response.code != "0" {
        return Err(anyhow!("okx error. {}, {}", response.code, response.msg));
    }

    Ok(response.data)
}

pub async fn close_position(
    ins_id: &str,
    pos_side: &str,
    mgn_mode: &str,
    ccy: &str,
) -> anyhow::Result<()> {
    let data = request::ClosePositionRequest {
        inst_id: ins_id.to_string(),
        pos_side: Some(pos_side.to_string()),
        mgn_mode: mgn_mode.to_string(),
        ccy: Some(ccy.to_string()),
        ..Default::default()
    };

    let data_json = serde_json::to_string(&data).unwrap();

    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let url_str = "/api/v5/trade/close-position";

    let header_map = okx_headers(
        &timestamp,
        &sign(&timestamp, METHOD_POST, url_str, &data_json)
            .await
            .unwrap(),
    )
    .await.map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;
    let url = format!("{}{}", API_BASE_URL, url_str);

    let resp = http::client()
        .await?
        .post(&url)
        .body(data_json.clone())
        .headers(header_map)
        .send()
        .await.map_err(|err| {
            error!("{}", err);
            anyhow!(err)
        })?;

    let text = resp.text().await.map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;
    
    info!("[{}] {}, {} => {}", METHOD_POST, url, data_json, text);
    
    let response: response::Response<Vec<response::ClosePositionResponse>> = serde_json::from_str(&text).map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;


    if response.code != "0" {
        return Err(anyhow!("okx error. {}, {}", response.code, response.msg));
    }

    Ok(())
}
