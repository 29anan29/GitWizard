import { zhCN } from './zh-CN';
import { en } from './en';

export type Locale = 'zh-CN' | 'en';

const dicts: Record<Locale, Record<string, string>> = {
	'zh-CN': zhCN,
	en
};

export const i18n = $state({ locale: 'zh-CN' as Locale });

export type Key = keyof typeof zhCN;

export function t<K extends keyof typeof zhCN>(
	key: K,
	params?: Record<string, string | number>
): string {
	const dict = dicts[i18n.locale] ?? zhCN;
	let text: string = dict[key as string] ?? zhCN[key];
	if (params) {
		for (const [k, v] of Object.entries(params)) {
			text = text.split(`{${k}}`).join(String(v));
		}
	}
	return text;
}
