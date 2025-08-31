// Don't show relm4::gtk 4.10 deprecations.
// We can't replace them without raising the relm4::gtk requirement to 4.10.
#![allow(deprecated)]

use std::convert::identity;

use relm4::gtk::glib;
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
        dialog = relm4::gtk::Dialog {
            #[watch]
            set_visible: model.visible,
            set_modal: true,

            #[wrap(Some)]
            set_child = &relm4::gtk::Label {
                set_width_request: 200,
                set_height_request: 200,
                set_halign: relm4::gtk::Align::Center,
                set_valign: relm4::gtk::Align::Center,
                #[watch]
                set_label: "Sozlamalar"
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

#[derive(Debug)]
enum ButtonMsg {}

#[relm4::component]
impl SimpleComponent for Button {
    type Init = ();
    type Input = ButtonMsg;
    type Output = AppMsg;

    view! {
        button = &relm4::gtk::Button {
            set_label: "Show the dialog",
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
        // We don't have access to the parent window from here
        // but we can just use the button to set the transient window for the dialog.
        // Relm4 will get the window later by calling [`WidgetExt::root()`]
        // on the button once all widgets are connected.
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
        main_window = relm4::gtk::ApplicationWindow {
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
    let app = RelmApp::new("relm4.example.transient_dialog");
    app.run::<App>(());
}
