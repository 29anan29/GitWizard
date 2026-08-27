<script lang="ts">
	import { fly } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import WizardNav from '$lib/components/WizardNav.svelte';
	import Button from '$lib/components/Button.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import circleCheck from '$lib/assets/icons/circle-check.svg?raw';
	import loader from '$lib/assets/icons/loader-2.svg?raw';
	import fileText from '$lib/assets/icons/file-text.svg?raw';
	import { repoStore, openDialog } from '$lib/state/repo.svelte';
	import { git } from '$lib/services/git';
	import { t } from '$lib/i18n/index.svelte';

	const CONFIG_FILES = [
		'.gitignore',
		'.gitattributes',
		'.editorconfig',
		'.github/workflows/ci.yml',
		'.github/workflows/release.yml'
	] as const;

	const GITIGNORE_TEMPLATES: Record<string, string> = {
		node: `node_modules/\n.env\n.env.local\ndist/\nbuild/\n.cache/\n`,
		python: `__pycache__/\n*.pyc\n*.pyo\n.env\nvenv/\n*.egg-info/\ndist/\nbuild/\n`,
		rust: `target/\nCargo.lock\n*.rs.bk\n`,
		go: `bin/\npkg/\nvendor/\n*.exe\n*.dll\n*.so\n*.dylib\n`
	};

	let existingFiles = $state<string[]>([]);
	let activeFile = $state<string | null>(null);
	let content = $state('');
	let busy = $state(false);
	let loadError = $state('');
	let saveError = $state('');
	let saved = $state(false);
	let showTemplates = $state(false);
	let newFileName = $state('');

	const info = $derived(repoStore.info);

	async function scanFiles(): Promise<void> {
		if (!info) return;
		existingFiles = [];
		for (const f of CONFIG_FILES) {
			try {
				await git.readFile(info.path, f);
				existingFiles = [...existingFiles, f];
			} catch {
				// file doesn't exist
			}
		}
	}

	async function openFile(path: string): Promise<void> {
		if (!info) return;
		busy = true;
		loadError = '';
		try {
			content = await git.readFile(info.path, path);
			activeFile = path;
		} catch (e) {
			loadError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	async function saveFile(): Promise<void> {
		if (!info || !activeFile) return;
		busy = true;
		saveError = '';
		saved = false;
		try {
			await git.writeFile(info.path, activeFile, content);
			saved = true;
			if (!existingFiles.includes(activeFile)) {
				existingFiles = [...existingFiles, activeFile];
			}
			setTimeout(() => (saved = false), 2000);
		} catch (e) {
			saveError = typeof e === 'string' ? e : String(e);
		} finally {
			busy = false;
		}
	}

	function applyTemplate(name: string): void {
		content = GITIGNORE_TEMPLATES[name] || '';
		newFileName = '.gitignore';
		activeFile = '.gitignore';
		showTemplates = false;
	}

	async function browse(): Promise<void> {
		try { await openDialog(); } catch { /* noop */ }
	}

	scanFiles();
</script>

<div class="container page">
	<WizardNav title={t('config.title')} steps={[t('config.title')]} current={1} {busy} oncancel={() => goto('/')} />

	{#if !info}
		<section class="guard">
			<h2>{t('guard.noRepo.title')}</h2>
			<p>{t('guard.noRepo.desc')}</p>
			<div class="actions">
				<Button variant="accent" onclick={browse}>{t('repo.openAction')}</Button>
				<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
			</div>
		</section>
	{:else}
		<section class="body" in:fly={{ y: 14, duration: 240, easing: cubicOut }}>
			{#if !activeFile}
				<div class="filelist-card">
					<span class="label">{t('config.existingFiles')}</span>
					{#if existingFiles.length === 0}
						<p class="empty">{t('config.noFiles')}</p>
					{:else}
						<ul class="filelist">
							{#each existingFiles as f (f)}
								<li>
									<button class="filelink" onclick={() => void openFile(f)} disabled={busy}>
										<Icon svg={fileText} size={14} />
										{f}
									</button>
								</li>
							{/each}
						</ul>
					{/if}
				</div>

				<div class="filelist-card">
					<span class="label">{t('config.createFromTemplate')}</span>
					<p class="hint">{t('config.templateHint')}</p>
					<div class="templates">
						{#each Object.keys(GITIGNORE_TEMPLATES) as name (name)}
							<button class="template-btn" onclick={() => applyTemplate(name)}>
								{name}
							</button>
						{/each}
					</div>
					{#if showTemplates}
						<div class="newfile-row">
							<input type="text" bind:value={newFileName} placeholder=".gitignore" spellcheck="false" />
							<Button variant="accent" disabled={!newFileName.trim()} onclick={() => { activeFile = newFileName.trim(); content = ''; }}>
								{t('config.create')}
							</Button>
						</div>
					{/if}
				</div>

				<footer class="foot">
					<Button variant="ghost" onclick={() => goto('/')}>{t('run.backHome')}</Button>
				</footer>
			{:else}
				<div class="toolbar">
					<span class="repo mono">{activeFile}</span>
					<div class="spacer"></div>
					{#if saved}
						<span class="saved-badge"><Icon svg={circleCheck} size={12} /> {t('config.saved')}</span>
					{/if}
					<Button variant="ghost" onclick={() => { activeFile = null; content = ''; }}>{t('common.back')}</Button>
					<Button variant="accent" disabled={busy} onclick={() => void saveFile()}>{t('common.save')}</Button>
				</div>

				<div class="editor-wrap">
					<textarea class="editor" spellcheck="false" bind:value={content}></textarea>
				</div>

				{#if loadError}
					<p class="errline">{loadError}</p>
				{/if}
				{#if saveError}
					<p class="errline">{saveError}</p>
				{/if}
			{/if}
		</section>
	{/if}
</div>

<style>
	.page { padding-bottom: 80px; }
	.guard { display: flex; flex-direction: column; align-items: center; gap: 10px; text-align: center; padding: 90px 24px; }
	.guard h2 { margin: 0; font-size: 26px; font-weight: 400; }
	.guard p { margin: 0 0 10px; font-family: var(--font-serif); color: var(--text-secondary); }
	.actions { display: flex; gap: 10px; }

	.body { max-width: 720px; margin: 28px auto 0; padding: 0 8px; }

	.filelist-card { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 16px 20px 14px; display: flex; flex-direction: column; gap: 10px; margin-bottom: 16px; }
	.label { font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.048px; color: var(--text-secondary); }
	.empty { margin: 0; font-size: 13px; color: var(--text-secondary); font-family: var(--font-serif); }
	.hint { margin: 0; font-size: 12px; color: var(--text-secondary); font-family: var(--font-serif); }

	.filelist { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 4px; }
	.filelist li { display: flex; }
	.filelink { display: inline-flex; align-items: center; gap: 8px; background: transparent; border: none; color: var(--color-text); font-family: var(--font-mono); font-size: 13px; padding: 6px 10px; border-radius: var(--radius-standard); cursor: pointer; text-align: left; transition: background 120ms ease; }
	.filelink:hover { background: var(--surface-100); }
	.filelink:disabled { opacity: 0.5; cursor: not-allowed; }

	.templates { display: flex; gap: 6px; flex-wrap: wrap; }
	.template-btn { background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-pill); padding: 5px 14px; font-size: 12.5px; cursor: pointer; transition: all 120ms ease; }
	.template-btn:hover { border-color: var(--color-accent); color: var(--color-accent); }

	.newfile-row { display: flex; align-items: center; gap: 8px; margin-top: 8px; }

	.toolbar { display: flex; align-items: center; gap: 10px; margin-bottom: 14px; }
	.repo { font-family: var(--font-mono); font-size: 13px; color: var(--text-secondary); }
	.spacer { flex: 1; }
	.saved-badge { display: inline-flex; align-items: center; gap: 5px; font-size: 12.5px; color: var(--color-success); background: rgba(31, 138, 101, 0.1); padding: 4px 10px; border-radius: var(--radius-pill); }

	.editor-wrap { background: var(--surface-400); box-shadow: rgba(38, 37, 30, 0.1) 0 0 0 1px inset; border-radius: var(--radius-featured); padding: 4px; margin-bottom: 12px; }
	.editor { width: 100%; min-height: 380px; border: none; background: transparent; font-family: var(--font-mono); font-size: 13px; line-height: 1.65; color: var(--color-text); resize: vertical; padding: 14px 16px; outline: none; box-sizing: border-box; }
	.editor::placeholder { color: var(--text-tertiary); }

	input[type='text'] { flex: 1; background: var(--surface-100); border: 1px solid var(--border-subtle); border-radius: var(--radius-standard); padding: 8px 11px; font-family: var(--font-mono); font-size: 13px; outline: none; }
	input:focus { border-color: var(--color-accent); }
	.mono { font-family: var(--font-mono); }

	.errline { margin: 0 0 12px; font-size: 12.5px; color: var(--color-error); white-space: pre-wrap; }
	.foot { display: flex; justify-content: center; margin-top: 24px; }
</style>
