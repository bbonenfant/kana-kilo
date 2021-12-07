//! These are helper methods to create the HTML of the various SVG icons used.
use yew::prelude::{Html, html};

const SVG_CLASS: &str = "svg-inline";
const SVG_FILL: &str = "currentColor";
const SVG_FOCUS: &str = "false";
const SVG_ROLE: &str = "img";
const SVG_XMLNS: &str = "http://www.w3.org/2000/svg";


pub fn clock_solid() -> Html {
    return html!{
        <svg
            data-icon="clock"
            focusable=SVG_FOCUS
            class=SVG_CLASS
            role=SVG_ROLE
            xmlns=SVG_XMLNS
            viewBox="0 0 512 512"
        >
            <path
                fill=SVG_FILL
                d="M256 512C114.6 512 0 397.4 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512zM232 256C232 264 236 271.5 242.7 275.1L338.7 339.1C349.7 347.3 364.6 344.3 371.1 333.3C379.3 322.3 376.3 307.4 365.3 300L280 243.2V120C280 106.7 269.3 96 255.1 96C242.7 96 231.1 106.7 231.1 120L232 256z"
            />
        </svg>
    }
}

pub fn github() -> Html {
    return html! {
        <svg
            role=SVG_ROLE
            viewBox="0 0 24 24"
            xmlns=SVG_XMLNS
        >
            <title>{ "Kana-Kilo Github Repo" }</title>
            <path
                fill=SVG_FILL
                d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
            />
        </svg>
    }
}

pub fn square_check_solid() -> Html {
    return html!{
        <svg
            data-icon="square-check"
            focusable=SVG_FOCUS
            class=SVG_CLASS
            role=SVG_ROLE
            xmlns=SVG_XMLNS
            viewBox="0 0 448 512"
        >
            <path
                fill=SVG_FILL
                d="M384 32H64C28.65 32 0 60.65 0 96v320c0 35.35 28.65 64 64 64h320c35.35 0 64-28.65 64-64V96C448 60.65 419.3 32 384 32zM339.8 211.8l-128 128C206.3 345.3 199.2 348 192 348s-14.34-2.719-19.81-8.188l-64-64c-10.91-10.94-10.91-28.69 0-39.63c10.94-10.94 28.69-10.94 39.63 0L192 280.4l108.2-108.2c10.94-10.94 28.69-10.94 39.63 0C350.7 183.1 350.7 200.9 339.8 211.8z"
            />
        </svg>
    }
}

pub fn square_solid() -> Html {
    return html!{
        <svg
            data-icon="square"
            focusable=SVG_FOCUS
            class=SVG_CLASS
            role=SVG_ROLE
            xmlns=SVG_XMLNS
            viewBox="0 0 448 512"
        >
            <path
                fill=SVG_FILL
                d="M448 95.1v320c0 35.35-28.65 64-64 64H64c-35.35 0-64-28.65-64-64v-320c0-35.35 28.65-63.1 64-63.1h320C419.3 31.1 448 60.65 448 95.1z"
            />
        </svg>
    }
}
