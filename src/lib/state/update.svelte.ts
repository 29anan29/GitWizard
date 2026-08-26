import { git, type UpdateInfo } from '$lib/services/git';

export const updateStore = $state({
	info: null as UpdateInfo | null,
	checked: false
});

let started = false;

export function startupUpdateCheck(proxy: string | null, enabled: boolean): void {
	if (started || !enabled) return;
	started = true;
	git
		.checkUpdates(proxy)
		.then((info) => {
			updateStore.info = info;
			updateStore.checked = true;
		})
		.catch(() => {
			updateStore.checked = true;
		});
}
