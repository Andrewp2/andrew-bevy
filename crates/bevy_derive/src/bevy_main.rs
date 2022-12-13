use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemFn};

pub fn bevy_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let boiler_plate = TokenStream::from(quote! {
        // use ndk-glue macro to create an activity: https://github.com/rust-windowing/android-ndk-rs/tree/master/ndk-macro
        #[cfg(target_os = "android")]
        #[cfg_attr(target_os = "android", bevy::ndk_glue::main(backtrace = "on", ndk_glue = "bevy::ndk_glue"))]
        fn android_main() {
            main()
        }

        #[no_mangle]
        #[cfg(target_os = "ios")]
        extern "C" fn main_rs() {
            main();
        }

        #[allow(unused)]
    });

    let mut output = vec![boiler_plate];
    assert!(
        input.sig.ident == "bevy_main",
        "`bevy_main` can only be used on a function called 'bevy_main'.",
    );
    if let Some(_) = input.sig.asyncness {
        input.sig.ident = Ident::new("main_async", input.sig.ident.span());
        let main = TokenStream::from(quote! {
            fn main() {
                bevy::tasks::run_async(main_async());
            }
        });
        output.push(main);
    } else {
        let item: TokenStream = input.to_token_stream().into();
        output.push(item);
    }
    TokenStream::from_iter(output)
}
