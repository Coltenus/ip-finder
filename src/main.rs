use thirtyfour::prelude::*;
use std::{fs, env, thread, process, path};
use regex::Regex;
use chrono::{Local, Timelike};
use sprintf::sprintf;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args: Vec<String> = env::args().collect();
    let mut first = true;
    let mut ip = "";
    let mut port = "";
    let mut op = 0;
    if env::args().count() == 4 {
        ip = &args[2];
        port = &args[1];
        op = args[3].parse::<u32>().unwrap();
    }
    else {
        first = false;
    }
    if !std::path::Path::new("chromedriver.exe").exists() {
        println!("No chromedriver in directory");
        first = false;
    }
    if first {
        let chromed = std::thread::spawn(move || {
            std::process::Command::new("./chromedriver.exe").output().expect("Chromedriver crashed");
        });
        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless()?;
        let mut server_url = String::from("http://localhost:");
        server_url.push_str(port);
        let driver = WebDriver::new(&server_url, caps).await?;
        let re = Regex::new(r"\d+\.\d+\.\d+\.\d+/\d+").unwrap();
        while first {
            driver.goto("https://gaia.cs.umass.edu/kurose_ross/interactive/subnet_addressing.php").await?;

            let list = driver.find(By::XPath("/html/body/div[1]/nav/button")).await;
            match list {
                Ok(b) => {
                    while !b.is_clickable().await? {}
                    b.click().await?;
                },
                Err(_) => {}
            }

            let button = driver.find(By::XPath("/html/body/div[1]/nav/div/ul/li[4]/button")).await;
            match button {
                Ok(b) => {
                    while !b.is_clickable().await? {}
                    b.click().await?;
                },
                Err(_) => {
                    println!("The button was not found.");
                }
            }

            let text = driver.source().await?;

            if let Some(m) = re.find(&text) {
                let mut Op = false;
                let mut Ch = "";
                let mut opStr: &str;
                match op {
                    2 => {
                        opStr = "(A and B)";
                    },
                    3 => {
                        opStr = "(A, B, and C)";
                    },
                    _ => {
                        opStr = "(A, B, and C)";
                    }
                }
                if text.contains(opStr) {
                    Op = true;
                    Ch = " +";
                }
                println!("Current ip - {}{}", m.as_str(), Ch);
                if m.as_str().contains(ip) && Op {
                    let dt = Local::now();
                    let name = sprintf!("O%d.%d.%d.html", dt.hour(), dt.minute(), dt.second());
                    match name {
                        Ok(t) => {
                            fs::write(t, text);
                        },
                        Err(_) => {
                            fs::write("output.html", text);
                        }
                    }
                    // first = false;
                }
            }
        }

        // chromed.join();
        driver.quit().await?;
    }

    Ok(())
}