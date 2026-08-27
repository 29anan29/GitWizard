import { git } from '$lib/services/git';
import type { Update as UpdaterUpdate } from '@tauri-apps/plugin-updater';

export const updateStore = $state({
	info: null as UpdaterUpdate | null,
	checked: false
});

let started = false;

export function startupUpdateCheck(_proxy: string | null, enabled: boolean): void {
	if (started || !enabled) return;
	started = true;
	git
		.checkUpdater()
		.then((upd) => {
			updateStore.info = upd;
			updateStore.checked = true;
		})
		.catch(() => {
			updateStore.checked = true;
		});
}
