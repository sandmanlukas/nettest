use std::error::Error;
use std::{thread, time, io::{stdout, Write}};


use headless_chrome::Browser;
use headless_chrome::LaunchOptionsBuilder;
//use headless_chrome::Element;


pub fn browser() -> Browser {
    Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
        .unwrap()
}


pub fn fetch_data() -> Result<(), Box<dyn Error>> {
    let browser = browser();

    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(60));

    tab.navigate_to("https://fast.com")?;

    let hundred_millis = time::Duration::from_millis(100);

    //let mut stdout = stdout();

    loop {

        let down_elem = tab.find_element("#speed-value")?;

        let down_unit_str = match tab.find_element("#speed-units") {
            Ok(elem)=> {
                let val = elem.call_js_fn("function() { return this.textContent;}",vec![], false)?.value.unwrap();
                val.as_str().unwrap().trim().replace("\"","")
            },
            Err(_e) => String::new(),
        };

        let down_val = down_elem.call_js_fn("function() { return this.textContent;}",vec![], false)?.value.unwrap();
        //let down_unit_val = down_unit_elem.call_js_fn("function() { return this.textContent;}",vec![], false)?.value.unwrap();

        let down_str = down_val.as_str().unwrap().trim().replace("\"", "");
        //let down_unit_str = down_unit_val.as_str().unwrap().trim().replace("\"","");


        print!("\rdown: {} {}", down_str, down_unit_str);
        stdout().flush().unwrap();
        //println!("up: {:#?}", up_speed);

        match tab.find_element("#speed-value.succeeded") {
            Ok(_done) =>  break,
            Err(_e) => (),
        };

        thread::sleep(hundred_millis);


    }
    println!();
    Ok(())

}
