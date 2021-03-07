use core::num;

use rand::prelude::*;
use gtk::prelude::*;
use gio::prelude::*;
fn main() {
    let application = gtk::Application::new(
        Some("com.github.prajwalprabhu.password-rs"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Password Generator");
        window.set_default_size(350, 70);
        let Box = gtk::Box::new(gtk::Orientation::Vertical,0);
        Box.add(& gtk::Label::new(Some("Enter Number of terms")));
        let entry = gtk::Entry::with_buffer(& gtk::EntryBuffer::new(None));
        Box.add(&entry);
        let button  = gtk::Button::with_label("Generate");
        Box.add(&button);
        // let button = Button::with_label("Click me!");
        button.connect_clicked(move |_| {
            let entry = entry.clone();
            let num = entry.get_text().to_string();
            let num_parse= num.trim().parse::<i32>();
            match num_parse{
                Ok(n)=> {
                    let ris = run(n);
                    // println!("risule {:?}",ris);
                    let mut ris_str = String::new();
                    for i in 0..ris.len(){
                        ris_str.push(ris[i]);
                    }
                    entry.set_text(&ris_str);
                    // for 
                    // let risult = String::from_;
                },
                Err(e)=>{entry.set_text(e.to_string().as_str());}
            }

            // println!("Num ; {:?}",);
        });
        window.add(&Box);

        window.show_all();
    });

    application.run(&[]);
}
pub fn run(num:i32)->Vec<char> {
    let values:[char;62] = ['q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m',
                                             'Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M',
                                             '1','2','3','4','5','6','7','8','9','0'];
    let mut rand = rand::thread_rng();
    let mut password:Vec<char> = Vec::new();
    for _ in 0..num{
        let i :usize =rand.gen_range(0..62);
        password.push(values[i]);
    }
    password
}
