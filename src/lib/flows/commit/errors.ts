import type { Key } from '$lib/i18n/index.svelte';

export function friendlyErrorKey(raw: string): Key {
	const r = raw.toLowerCase();
	if (/cred_missing/.test(r)) return 'err.noCred';
	if (/branch_current/.test(r)) return 'err.branchCurrent';
	if (/branch_unmerged/.test(r)) return 'err.branchUnmerged';
	if (/branch_name/.test(r)) return 'err.branchName';
	if (/branch_exists/.test(r)) return 'err.branchExists';
	if (/branch_not_found/.test(r)) return 'err.branchNotFound';
	if (/non-fast-forward|fetch first|rejected|non_ff/.test(r)) return 'err.nonff';
	if (/\b401\b|authentication|credential|auth fail/.test(r)) return 'err.auth';
	if (/\b403\b|permission|forbidden/.test(r)) return 'err.forbidden';
	if (/repository not found/.test(r)) return 'err.repoNotFound';
	if (/找不到远端|remote.*not.?found|origin.*not found/.test(r)) return 'err.noRemote';
	if (/^proxy:|proxy/.test(r)) return 'err.proxy';
	if (/ssl|tls|certificate/.test(r)) return 'err.ssl';
	if (/resolve host|connection|network|timed? ?out|unreachable|^net:|^http:|dns/i.test(r))
		return 'err.network';
	return 'err.generic';
}
