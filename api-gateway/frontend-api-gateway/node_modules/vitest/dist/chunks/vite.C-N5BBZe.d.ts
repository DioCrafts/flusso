import { I as InlineConfig } from './reporters.D7Jzd9GS.js';

type VitestInlineConfig = InlineConfig;
declare module 'vite' {
    interface UserConfig {
        /**
         * Options for Vitest
         */
        test?: VitestInlineConfig;
    }
}
