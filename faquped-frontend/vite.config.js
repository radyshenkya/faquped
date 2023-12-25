import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ command, mode }) => {
    const env = loadEnv(mode, '../', '');

    return {
        server: {
            port: Number(env.VITE_PORT),
            proxy: {
                '/api': 'http://' + env.SERVER_IP
            }
        }
    };
});
