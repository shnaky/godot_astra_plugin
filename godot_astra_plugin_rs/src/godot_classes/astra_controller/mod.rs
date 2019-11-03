use astra;
use gdnative::init::{Property, PropertyHint, PropertyUsage};
use gdnative::*;

mod color;
use color::ColorState;
mod body;
mod masked_color;

pub struct AstraController {
    sensor: astra::Sensor,
    reader: astra::Reader,
    body_frame_index: i32,
    body_fps: u32,
    body_stream: Option<astra::Stream>,
    color: ColorState,
    masked_color: ColorState,
}

unsafe impl Send for AstraController {}

impl NativeClass for AstraController {
    type Base = Node;
    type UserData = user_data::MutexData<AstraController>;

    fn class_name() -> &'static str {
        "AstraController"
    }

    fn init(_owner: Self::Base) -> Self {
        unsafe { Self::_init(_owner) }
    }

    fn register_properties(builder: &init::ClassBuilder<Self>) {
        builder.add_property(Property {
            name: "body_fps",
            default: 30,
            hint: PropertyHint::Range {
                range: 0.0..60.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &AstraController| this.body_fps,
            setter: |this: &mut AstraController, v| this.body_fps = v,
            usage: PropertyUsage::DEFAULT,
        });

        builder.add_property(Property {
            name: "color_fps",
            default: 30,
            hint: PropertyHint::Range {
                range: 0.0..60.0,
                step: 1.0,
                slider: true,
            },
            getter: |this: &AstraController| this.color.fps,
            setter: |this: &mut AstraController, v| this.color.fps = v,
            usage: PropertyUsage::DEFAULT,
        });

        builder.add_signal(init::Signal {
            name: "new_body_list",
            args: &[init::SignalArgument {
                name: "body_list",
                default: Variant::from_array(&VariantArray::new()),
                hint: init::PropertyHint::None,
                usage: init::PropertyUsage::DEFAULT,
            }],
        });
        // This event will cause editor crash sometimes
        builder.add_signal(init::Signal {
            name: "new_color_byte_array",
            args: &[
                init::SignalArgument {
                    name: "width",
                    default: Variant::from(0_u64),
                    hint: init::PropertyHint::None,
                    usage: init::PropertyUsage::DEFAULT,
                },
                init::SignalArgument {
                    name: "height",
                    default: Variant::from(0_u64),
                    hint: init::PropertyHint::None,
                    usage: init::PropertyUsage::DEFAULT,
                },
                init::SignalArgument {
                    name: "image",
                    default: Variant::from_object(&Image::new()),
                    hint: init::PropertyHint::None,
                    usage: init::PropertyUsage::DEFAULT,
                },
            ],
        });
    }
}

#[methods]
impl AstraController {
    /// The "constructor" of the class.
    unsafe fn _init(_owner: Node) -> Self {
        astra::init();
        let sensor = astra::get_sensor();
        let reader = astra::get_reader(sensor);
        AstraController {
            sensor: sensor,
            reader: reader,
            body_frame_index: -1,
            body_stream: None,
            color: ColorState {
                fps: 30,
                ..Default::default()
            },
            masked_color: ColorState {
                fps: 30,
                ..Default::default()
            },
            body_fps: 30,
        }
    }

    #[export]
    unsafe fn _exit_tree(&mut self, _owner: Node) {
        if let Some(stream) = self.body_stream {
            astra::stop_stream(stream);
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Node) {
        self.body_stream = Some(self.start_body_stream(owner));
        //self.start_color_stream(owner);
    }

    #[export]
    unsafe fn update_color(&mut self, owner: Node) {
        self.handle_update_color(owner);
    }

    #[export]
    unsafe fn update_masked_color(&mut self, owner: Node) {
        self.handle_update_masked_color(owner);
    }

    #[export]
    unsafe fn update_body(&mut self, owner: Node) {
        self.handle_update_body(owner);
    }
}