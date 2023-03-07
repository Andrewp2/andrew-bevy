use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemFn};

pub fn bevy_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    // assert!(
    //     input.sig.ident == "main",
    //     "`bevy_main` can only be used on a function called 'main'.",
    // );

    let boilerplate = TokenStream::from(quote! {
        #[no_mangle]
        #[cfg(target_os = "android")]
        fn android_main(android_app: bevy::winit::AndroidApp) {
            let _ = bevy::winit::ANDROID_APP.set(android_app);
            main();
        }

        #[no_mangle]
        #[cfg(target_os = "ios")]
        extern "C" fn main_rs() {
            main();
        }

        #[allow(unused)]
        #input
    });
    let mut output = vec![boilerplate];
    if let Some(_) = input.sig.asyncness {
        input.sig.ident = Ident::new("main_async", input.sig.ident.span());
        let main = TokenStream::from(quote! {
            fn main() {
                bevy::tasks::run_async(main_async());
            }
        });
        output.push(main);
    }
    let item: TokenStream = input.to_token_stream().into();
    output.push(item);
    TokenStream::from_iter(output)
}
