use std::{error::Error,thread::sleep, time::Duration};
use terminal_spinners::{SpinnerBuilder, DOTS};

use headless_chrome::Browser;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::Element;

pub fn browser() -> Browser {
    Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
        .unwrap()
}


fn get_str_from_element(elem: Result<Element, Box<dyn Error>>) -> String {
    match elem {
        Ok(elem) => {
            let val = elem.call_js_fn("function() { return this.textContent;}", vec![], false).unwrap().value.unwrap();
            let text = val.as_str().unwrap().trim().replace("\"","");

            if text == "0" {
                return "-".to_string();
            } else {
                return text;
            }

        },
        Err(_e) => return String::new(),
    }
}


pub fn fetch_data(more: bool) -> Result<(), Box<dyn Error>> {
    let browser = browser();

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://fast.com")?;

    let hundred_millis = Duration::from_millis(100);
    let handle = SpinnerBuilder::new().spinner(&DOTS).text("").start();

    loop {

        // Add some sort of error check here, if the loop has passed x amounts of time, break.

        let down_speed_elem = match tab.find_element("#speed-value") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let up_speed_elem = match tab.find_element("#upload-value") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let down_units_elem = match tab.find_element("#speed-units") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let up_units_elem = match tab.find_element("#upload-units") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let latency_units_elem = match tab.find_element("#latency-units") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let bufferbloat_units_elem = match tab.find_element("#bufferbloat-units") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let downloaded_elem = match tab.find_element("#down-mb-value") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let uploaded_elem = match tab.find_element("#up-mb-value") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let latency_elem = match tab.find_element("#latency-value") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let buffer_bloat_elem = match tab.find_element("#bufferbloat-value") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };


        let location_elem = match tab.find_element("#user-location") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let server_location_elem = match tab.find_element("#server-locations") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let ip_elem = match tab.find_element("#user-ip") {
            Ok(elem) => elem,
            Err(_e) => continue,
        };

        let down_speed = get_str_from_element(Ok(down_speed_elem));
        let up_speed = get_str_from_element(Ok(up_speed_elem));
        let down_unit = get_str_from_element(Ok(down_units_elem));
        let up_unit = get_str_from_element(Ok(up_units_elem));
        let latency_unit = get_str_from_element(Ok(latency_units_elem));
        let bufferbloat_unit = get_str_from_element(Ok(bufferbloat_units_elem));
        let downloaded = get_str_from_element(Ok(downloaded_elem));
        let uploaded = get_str_from_element(Ok(uploaded_elem));
        let latency = get_str_from_element(Ok(latency_elem));
        let buffer_bloat = get_str_from_element(Ok(buffer_bloat_elem));
        let location = get_str_from_element(Ok(location_elem));
        let server_location = get_str_from_element(Ok(server_location_elem));
        let ip = get_str_from_element(Ok(ip_elem));



        let terminal_str = format!(" {} {} ↓ / {} {} ↑", down_speed, down_unit, up_speed, up_unit);
        handle.text(terminal_str);

        match (tab.find_element("#speed-value.succeeded"),tab.find_element("#upload-value.succeeded")) {
            (Ok(_down), Ok(_up)) =>  {
                let closed = tab.close(true)?;
                assert_eq!(closed, true);

                handle.done();

                if more {
                    println!("Downloaded: {} MB", downloaded);
                    println!("Uploaded: {} MB", uploaded);
                    println!("Latency: {} {}", latency, latency_unit);
                    println!("Bufferbloat: {} {}", buffer_bloat, bufferbloat_unit);
                    println!("Client: {} {}",ip, location);
                    println!("Server/servers: {}", server_location);
                }

                break
            },

            _ => (),
        };

        sleep(hundred_millis);


    }

    Ok(())

}
