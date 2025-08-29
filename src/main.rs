use gtk::prelude::*;
use relm4::adw::prelude::*;
use relm4::prelude::*;
struct AppModel {
    counter: u8,
    expander: adw::ExpanderRow
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                set_vexpand: true,
                set_hexpand: true,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => AppMsg::Increment,
                },

                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked => AppMsg::Decrement,
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                },

                gtk::Box{
                    adw::PreferencesGroup {
                        set_title: "Extensions",

                        #[name(expander)]
                        add = &adw::ExpanderRow {
                            set_title: "AppIndicator and KStatusNotifierItem Support",
                            set_subtitle: "appindicatorsupport@rgcjonas.gmail.com",
                        }
                    }


                }



            }
        }
    }

    // Initialize the component.
    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = AppModel { 
            counter,
            expander: adw::ExpanderRow::new()
        };
        // Insert the code generation of the view! macro here
        let widgets = view_output!();
        model.expander = widgets.expander.clone();
        let justify = gtk::Justification::Left;
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        vbox.set_align(gtk::Align::Start);
        
        for i in 1..5 {
            // if i == 1{ 
            // }
            let label = &gtk::Label::builder().label("sadasdasd").justify(justify).build();
            vbox.append(label);
            model.expander.add_row(&vbox);         
        }
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<AppModel>(0);
}
