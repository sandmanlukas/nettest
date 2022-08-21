mod api;


fn main(){

   match api::fetch_data() {
       Ok(()) => println!("done!"),
       Err(e) => panic!("error e: {}", e),
   }

}
