<script lang="ts">
    import {t} from "$lib/i18n/i18n";
    import {pickAppImage} from "$lib/helpers/tauriCommands/dialogCommands";
    import {installAppImage} from "$lib/helpers/tauriCommands/appImageCommands";
    import Modal from "$lib/components/Modal.svelte";
    import LoadingOverlay from "$lib/components/LoadingOverlay.svelte";
    import { fade } from "svelte/transition";
    import {onMount} from "svelte";
    import {readAppsDatabase, updateAppsDatabase} from "$lib/helpers/tauriCommands/appDatabaseCommands";

    let appPath: string | undefined = undefined;
    let enableAdvancedOptions: boolean = false;

    let noSandbox: boolean = false;

    let modalOpen: boolean = false;
    let modalTitle: string;
    let modalMessage: string;
    let isLoading: boolean = false;

    const chooseFile = async () => {
        try {
            appPath = await pickAppImage();
        } catch (e) {
            console.error(e);
            modalOpen = true;
            modalTitle = $t("install_file.error_modal_title");
            modalMessage = $t("install_file.error_modal_choose_file");
        }
    }

    const installApp = async () => {
        isLoading = true;
        try {
            if (!appPath) {
                throw new Error("No app path selected");
            }
            const res = await installAppImage(
                appPath,
                noSandbox
            )
            console.log("Installation result", res);
            modalOpen = true;
            modalTitle = $t("install_file.success_modal_title");
            modalMessage = $t("install_file.installation_successful");
        } catch (e) {
            console.error(e);
            modalOpen = true;
            modalTitle = $t("install_file.error_modal_title");
            modalMessage = $t("install_file.error_modal_install");
        } finally {
            isLoading = false;
            appPath = undefined;
        }
    }

    onMount(() => {
        console.log("Page mounted");
        updateAppsDatabase().then((res)=>{
            readAppsDatabase().then((res)=>{
                console.log("Apps database", res);
            })
        })
    });

</script>

<div class="flex flex-col bg-base-200 rounded-box mx-10 mt-10 p-5" in:fade={{duration: 500}}>
    {#if !!appPath}
        <div class="mt-3">
            <div class="container mx-auto px-4 py-4">
                <p class="font-bold text-2xl">{$t("install_file_selected_title")}</p>
                <p>{$t("install_file_selected_des")} {appPath}</p>

                <div class="mt-4 form-control w-1/5">
                    <label class="label cursor-pointer">
                        <span class="label-text">{$t("install_file_advanced_accordio_title")}</span>
                        <input bind:checked={enableAdvancedOptions} type="checkbox"
                               class="checkbox"/>
                    </label>
                </div>
            </div>

            {#if enableAdvancedOptions}
                <div class="container mx-auto px-4 mt-3">
                    <div class="flex flex-col justify-start items-start mt-4 space-y-2">
                        <div class="tooltip tooltip-right"
                             data-tip={$t("install_file.advanced_options.no_sandbox_des")}>
                            <p class="mb-1">{$t("install_file.advanced_options.title")}</p>
                        </div>
                        <label class="inline-flex items-center">
                            <input bind:checked={noSandbox} type="checkbox" class="checkbox"/>
                            <span class="ml-2">{$t("install_file.advanced_options.no_sandbox")}</span>
                        </label>
                    </div>
                </div>
            {/if}

        </div>
    {:else }
        <div class="container mx-auto px-4 py-8">
            <h2 class="font-bold text-3xl mb-4">{$t("install_choose_file")}</h2>
            <p class="text-lg mb-6">{$t("install_choose_file")}</p>
            <button on:click={chooseFile} class="btn btn-primary">
                {$t("install_choose_button")}
            </button>
        </div>
    {/if}

</div>
<div class="my-5 mx-10">
    <button on:click={installApp}
            class="btn btn-success w-full"
            disabled="{!appPath}">
        {$t("install_file_install_button")}
    </button>
</div>

<Modal modalOpen={modalOpen} closeCallback={()=>{modalOpen = false}}>
    <div class="flex flex-col">
        <p class="text-2xl font-bold">{modalTitle}</p>
        <p class="mt-3">{modalMessage}</p>
    </div>
</Modal>

<LoadingOverlay loading={isLoading}/>
