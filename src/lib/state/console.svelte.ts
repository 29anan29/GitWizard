import { onLog, onPushProgress } from '$lib/services/git';

export interface ConsoleLine {
	kind: 'cmd' | 'out' | 'err';
	line: string;
}

export const consoleStore = $state({
	lines: [] as ConsoleLine[],
	open: false,
	pushPercent: -1
});

let attached = false;

export function attachConsole(): void {
	if (attached) return;
	attached = true;
	onLog((l) => {
		consoleStore.lines.push({ kind: l.kind ?? 'out', line: l.line });
		trim();
	}).catch(() => {});
	onPushProgress((p) => {
		consoleStore.pushPercent = p.percent;
	}).catch(() => {});
}

function trim(): void {
	if (consoleStore.lines.length > 600) {
		consoleStore.lines.splice(0, consoleStore.lines.length - 600);
	}
}

export function toggleConsole(): void {
	consoleStore.open = !consoleStore.open;
}

export function clearConsole(): void {
	consoleStore.lines = [];
}
