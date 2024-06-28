use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{bail, Result};
use reqwest;

const MAP: &[u8] = concat!(r##"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!"#$%&'()*+,-./:;<=>?@[\]^_`|~ "##, '\n').as_bytes();

fn cult_to_human(s: &[u8]) -> Result<String> {
    let mut chs = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        if s[i] < b'!' || s[i] > b'~' {
            bail!("Invalid character: {}", s[i]);
        }
        chs.push(MAP[(s[i] - b'!') as usize]);
    }
    Ok(String::from_utf8(chs)?)
}

fn human_to_cult(s: &str) -> Result<Vec<u8>> {
    let mut chs = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        let mut found = false;
        for j in 0..MAP.len() {
            if MAP[j] == s.as_bytes()[i] {
                chs.push(j as u8 + b'!');
                found = true;
                break;
            }
        }
        if !found {
            bail!("Invalid character: {}", s.as_bytes()[i]);
        }
    }
    Ok(chs)
}

async fn post(body: &[u8]) -> Result<String> {
    let token = std::env::var("TOKEN").unwrap();
    let client = reqwest::Client::new();
    let res = client.post("https://boundvariable.space/communicate")
        .header("Authorization", format!("Bearer {}", token))
        .body(body.to_vec())
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}

#[get("/{msg}")]
async fn comm(msg: web::Path<String>) -> impl Responder {
    let mut body = human_to_cult(&msg).unwrap();
    body.insert(0, b'S');
    let resp = post(&body).await.unwrap();
    if resp.as_bytes()[0] != b'S' {
        return HttpResponse::BadRequest().body("not string: ".to_string() + &resp);
    }
    let resp_human = cult_to_human(resp[1..].as_bytes()).unwrap();
    HttpResponse::Ok().body(resp_human)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(comm)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
