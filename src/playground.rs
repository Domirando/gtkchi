use relm4::adw::prelude::*;
use relm4::prelude::*;

struct AppModel {
    clicked: bool,
    expander: adw::ExpanderRow
}

#[derive(Debug)]
enum AppMsg {
    Settings,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = bool;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,
                set_margin_all: 20,
                set_vexpand: true,
                set_hexpand: true,

                gtk::Box{

                    adw::PreferencesGroup {
                        #[name(expander)]
                        add = &adw::ExpanderRow {
                            set_title: "AppIndicator and KStatusNotifierItem Support",
                            set_subtitle: "appindicatorsupport@rgcjonas.gmail.com",
                        }
                    }
                },
               gtk::Button {
                    set_label: "Sozlamalar",
                    connect_clicked => AppMsg::Settings,
               },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Bosildi: {}", model.clicked),
                    set_margin_all: 5,
                },
            }
        }
    }

    // Initialize the component.
    fn init(
        clicked: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = AppModel { 
            clicked,
            expander: adw::ExpanderRow::new()
        };
        // Insert the code generation of the view! macro here
        let widgets = view_output!();
        model.expander = widgets.expander.clone();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        vbox.set_align(gtk::Align::Start);
        
//         for i in 1..5 {
//             // if i == 1{
//             // }
//             let label = &gtk::Label::builder().label("sadasdasd").justify(justify).build();
//             vbox.append(label);
//             model.expander.add_row(&vbox);
//         }
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Settings => {
                self.clicked = !self.clicked;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple");
    app.run::<AppModel>(false);
}



