use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::prelude::*;
use leptonic::components::prelude::*;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageIcon() -> impl IntoView {
    view! {
        <Article>
            <H1 id="icon" class="anchor">
                "Icon"
                <AnchorLink href="#icon" description="Direct link to article header"/>
            </H1>

            <P>
                "Icons are supported through the "<LinkExt target=LinkExtTarget::Blank href="https://crates.io/crates/icondata">"https://crates.io/crates/icondata"</LinkExt> " crate. "
                "The crates readme show the available icon packages and their current versions."
            </P>

            <P style="font-weight: bold;">"Browse through available icons at "<LinkExt target=LinkExtTarget::Blank href="https://carlosted.github.io/icondata/">"https://carlosted.github.io/icondata/"</LinkExt> "!"</P>

            <P>"Leptonic provides a re-export of the "<Code inline=true>"icondata"</Code>" crate when using the prelude module."</P>

            <P>"You can simply include an icon using the "<Code inline=true>"<Icon>"</Code>" component."</P>

            <Code>
                {indoc!(r#"
                    use leptonic::prelude::*;

                    view! {
                        <Icon icon=icondata::BsFolderFill style="font-size: 6em;"/>
                        <Icon icon=icondata::BsFolder style="font-size: 6em;"/>
                    }
                "#)}
            </Code>

            <Icon icon=icondata::BsFolderFill style="font-size: 6em;"/>
            <Icon icon=icondata::BsFolder style="font-size: 6em;"/>

            <P>"SVG data of all the icons used in your application is embedded into your binary."</P>

            <H2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </H2>

            <P>"There are currently no CSS variables exposed, targeting the "<Code inline=true>"<Icon>"</Code> " component."</P>

            <P>"Notes:"</P>
            <ul>
                <li>"Scale icons by setting the css "<Code inline=true>"font-size"</Code> " attribute."</li>
                <li>"Color icons by setting the css "<Code inline=true>"color"</Code> " attribute."</li>
                <li>"Some icons may also render different when altering the css "<Code inline=true>"background-color"</Code> " attribute."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Icon", link: "#icon" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
