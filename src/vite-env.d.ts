/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_REPO_LINK_DEV: string;
    readonly VITE_REPO_LINK_PUBLIC: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
