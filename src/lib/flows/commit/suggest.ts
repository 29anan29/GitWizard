import { t } from '$lib/i18n/index.svelte';
import type { Locale } from '$lib/i18n/index.svelte';

export function suggestMessage(paths: string[], locale: Locale): string {
	const n = paths.length;
	const docs = paths.filter(isDoc);
	const tests = paths.filter(isTest);
	const confs = paths.filter(isConfig);
	const codeCount = n - docs.length - tests.length - confs.length;

	const names = shortNames(
		codeCount > 0 ? paths.filter((p) => !isDoc(p) && !isTest(p) && !isConfig(p)) : paths
	);

	if (docs.length / n >= 0.6) return t('suggest.docs', { n: docs.length });
	if (tests.length === n && tests.length > 0)
		return t('suggest.test', { list: shortNames(tests).join(', ') });
	if (confs.length === n && confs.length > 0)
		return t('suggest.choreConfig', { list: shortNames(confs).join(', ') });
	if (codeCount / n >= 0.6)
		return t('suggest.feat', { n: codeCount, list: names.slice(0, 3).join(', ') });
	return t('suggest.choreMixed', { n });
}

function shortNames(paths: string[]): string[] {
	return paths.slice(0, 3).map((p) => {
		const parts = p.split('/');
		return parts[parts.length - 1];
	});
}

function isDoc(p: string): boolean {
	return /\.(md|txt|rst|adoc|pdf)$/i.test(p) || /(^|\/)(docs?|documentation)\//i.test(p);
}

function isTest(p: string): boolean {
	return (
		/(^|\/)(tests?|__tests__|spec)\//i.test(p) ||
		/\.(test|spec)\.[jt]sx?$/i.test(p) ||
		/test_[^/]*\.py$/i.test(p)
	);
}

function isConfig(p: string): boolean {
	return /\.(ya?ml|toml|json|ini|lock)$/.test(p) || /(^|\/)\.(env|gitignore|editorconfig)/.test(p);
}
