import { sveltekit } from '@sveltejs/kit/vite';
import { build, defineConfig, type UserConfigExport  } from 'vite';

export default defineConfig(
	()=>{
		const retval:UserConfigExport =
		{
			plugins: [sveltekit()],
		};

		return retval;
	}
);
