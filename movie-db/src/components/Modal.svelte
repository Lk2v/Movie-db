<script lang="ts">
    let {
        title,
        desciption,
        body,
        footer
    } : {
        title: string;
        desciption?: string;
        body: () => any;
        footer: () => any;
    } = $props();
</script>

<div class="modal">
    <div class="modal-header">
        <h3>{title}</h3>
        {#if desciption}
            <subtitle>{desciption}</subtitle>
        {/if}
    </div>
    <div class="modal-content">
        {@render body()}
    </div>
    <div class="modal-footer">
        {@render footer()}
    </div>
</div>

<style>
    .modal {
        background-color: var(--color-background-tertiary);

        min-width: 400px;
        min-height: 200px;

        border-radius: 8px;
        padding: 1rem;

        box-shadow: 0 0 10px rgba(0,0,0,0.1);

        flex-direction: column;
        display: flex;
    }

    .modal-header {
        margin-bottom: 1rem;
    }

    .modal-content {
        padding: 1rem 0;
        flex-grow: 1;

        overflow-y: auto;

        justify-content: flex-start;
        flex-direction: column;
        display: flex;
    }

    .modal-footer {
        margin-top: 1rem;

        justify-content: flex-end;
        flex-direction: row;
        display: flex;
    }
</style>