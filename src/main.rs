#![allow(deprecated)]
use std::convert::identity;
use relm4::gtk::glib;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use relm4::gtk::prelude::{ButtonExt, GtkWindowExt, WidgetExt};
use relm4::MessageBroker;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};
static DIALOG_BROKER: MessageBroker<DialogMsg> = MessageBroker::new();

struct Dialog {
    visible: bool,
}

#[derive(Debug)]
enum DialogMsg {
    Show,
    Hide,
}

#[relm4::component]
impl SimpleComponent for Dialog {
    type Init = ();
    type Input = DialogMsg;
    type Output = ButtonMsg;

    view! {
        dialog = gtk::Dialog {
            add_css_class: "wrap",
            #[watch]
            set_visible: model.visible,
            set_modal: true,

            #[wrap(Some)]
            set_child = &relm4::gtk::Box {
                set_margin_all: 20,
                set_spacing: 20,
                set_width_request: 350,
                set_orientation: gtk::Orientation::Vertical,
                gtk::Box {
                    set_spacing: 5,
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Image {
                    set_pixel_size: 150,
                    set_paintable: Some(&embedded_logo()),
                },
                gtk::Label {
                    set_halign: gtk::Align::Center,
                    add_css_class: "title",
                    set_label: "Sozlamalar"
                },
                gtk::Label {
                    set_halign: gtk::Align::Center,
                    set_label: "The Gnome Project"
                },
                gtk::Button{
                    add_css_class: "btn-version",
                    set_label: "48.2",  
                    set_hexpand: false,  
                    set_halign: gtk::Align::Center,   
                },
                },
                
                gtk::Box {
                    add_css_class: "box",
                    set_hexpand: true,
                    set_width_request: 300,
                    gtk::Label {
                        set_justify: gtk::Justification::Left,
                        add_css_class: "box-label",
                        set_align: gtk::Align::Start,
                        set_hexpand: false,
                        // set_halign: gtk::Align::Start,
                        set_label: "Website"
                    },
                    gtk::Image {
                        add_css_class: "icon",
                        set_hexpand: false,
                        set_align: gtk::Align::End,
                        // set_halign: gtk::Align::End,
                        set_icon_name: Some("send-to-symbolic"),
                    },
                },
            
                gtk::Box{
                    set_spacing: 1,
                    set_orientation: gtk::Orientation::Vertical,
                    gtk::Box {
                    add_css_class: "box-top",
                    set_hexpand: true,
                    set_width_request: 300,
                    gtk::Label {
                        set_justify: gtk::Justification::Left,
                        add_css_class: "box-label",
                        // set_halign: gtk::Align::Start,
                        set_label: "Support Qestions"
                    },
                    gtk::Image {
                        add_css_class: "icon",
                        // set_halign: gtk::Align::End,
                        set_icon_name: Some("send-to-symbolic"),
                    },
                },
                gtk::Box {
                        add_css_class: "box-bottom",
                        set_hexpand: true,
                        set_width_request: 300,
                        gtk::Label {
                            set_justify: gtk::Justification::Left,
                            add_css_class: "box-label",
                            // set_halign: gtk::Align::Start,
                            set_label: "Report an Issue"
                        },
                        gtk::Image {
                            add_css_class: "icon",
                            // set_halign: gtk::Align::End,
                            set_icon_name: Some("send-to-symbolic"),
                        },
                    },
                
                },
            
                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 1,
                    gtk::Box {
                    add_css_class: "box-top",
                    set_hexpand: true,
                    set_width_request: 300,
                    gtk::Label {
                        set_justify: gtk::Justification::Left,
                        add_css_class: "box-label",
                        // set_halign: gtk::Align::Start,
                        set_label: "Credits"
                    },
                    gtk::Image {
                        add_css_class: "icon",
                        // set_halign: gtk::Align::End,
                        set_icon_name: Some("send-to-symbolic"),
                    },
                },
                gtk::Box {
                        add_css_class: "box-bottom",
                        set_hexpand: true,
                        set_width_request: 300,
                        gtk::Label {
                            set_justify: gtk::Justification::Left,
                            add_css_class: "box-label",
                            // set_halign: gtk::Align::Start,
                            set_label: "Legal"
                        },
                        gtk::Image {
                            add_css_class: "icon",
                            // set_halign: gtk::Align::End,
                            set_icon_name: Some("send-to-symbolic"),
                        },
                    },
                
                }
            
            },



            connect_close_request[sender] => move |_| {
                sender.input(DialogMsg::Hide);
                glib::Propagation::Stop
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Dialog { visible: false };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show => self.visible = true,
            DialogMsg::Hide => self.visible = false,
        }
    }
}

struct Button {
    #[allow(dead_code)]
    dialog: Controller<Dialog>,
}

fn embedded_logo() -> gtk::gdk::Texture {
    let bytes = include_bytes!("../assets/settings.png");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = gtk::gio::MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk::gio::Cancellable::NONE).unwrap();
    gtk::gdk::Texture::for_pixbuf(&pixbuf)
}

#[derive(Debug)]
enum ButtonMsg {}

#[relm4::component]
impl SimpleComponent for Button {
    type Init = ();
    type Input = ButtonMsg;
    type Output = AppMsg;

    view! {
        button = gtk::Button {
            set_label: "Sozlamalaaaar",
            connect_clicked => move |_| {
                DIALOG_BROKER.send(DialogMsg::Show);
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dialog = Dialog::builder()
            //.transient_for(&root)
            .launch_with_broker((), &DIALOG_BROKER)
            .forward(sender.input_sender(), identity);

        let model = Button { dialog };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}

#[derive(Debug)]
enum AppMsg {}

struct App {
    button: Controller<Button>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_default_size: (500, 250),
            set_child: Some(model.button.widget()),
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let button = Button::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        let model = App { button };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}

fn main() {
    gtk::init().expect("noo");
    let provider = gtk::CssProvider::new();
    provider.load_from_path("./assets/style.css");
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let app = RelmApp::new("relm4.example.transient_dialog");
    app.run::<App>(());
}
