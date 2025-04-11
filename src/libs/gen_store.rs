#[macro_export]
macro_rules! gen_store {
    ( $($name:ident : $ty:tt,)* ) => {
        #[derive(Clone, Copy)]
        pub struct Store {
            (pub $name: ReadSignal<$ty>,)*
        }

        pub fn use_store(url: &str) -> Result<Store, JsError> {
            let ws = use_web_socket(url)?;
            let x = ws.message();

            (let $name = create_signal::<$ty>($ty::default());)*

            create_memo(move|| {
                let act = serde_json::from_str::<Message>(&x.get_clone())
                    .unwrap_or_else(|_| Message::default());
                match act {
                    (Message{content: Content::$name(x), ..} => $name.set(x),)*
                    Message{user: _, content: Content::empty} => (),
                };
            });

            Ok(Store{
                ($name: *name,)*
            })
        }
    };
}

//gen_store!{ layout: Layout, };
