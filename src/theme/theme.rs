use stylers::style;

pub const APP_CLASS: &str = style! {"App",
    :root {
        --black: #000;
        --purple: #5c2dd5;
        --purple-light: #7945ff;
        --pink: #fd6687;
        --yellow: #ffce67;
        --white: #fff;
        --text-large: 56px;
        --text-medium: 24px;
        --text-small: 20px;
        --text-extra-small: 16px;
        --font-family: "Space Grotesk", sans-serif;
        padding: 0;
        margin: 0;
    }
    * {
        font-family: var(--font-family);
    }
    .app {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        background-color: var(--purple-light);
        color: var(--white);
        padding: 0;
        margin: 0;
    }

};
