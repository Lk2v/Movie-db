<script lang="ts">
    let {
        label,
        value = $bindable(""),
        placeholder = "",

        autocapitalize = "on",
        password = false,

        disabled = false,
    }: {
        label?: string;
        value: string;
        placeholder?: string;

        autocapitalize?: "on" | "off";
        password?: boolean;

        disabled?: boolean;
    } = $props();
</script>

<div class="text-field-container">
    {#if label}
        <span class="label">{label}</span>
    {/if}
    <div class="text-field">
        <input 
            type={password ? "password" : "text"}
            bind:value
            placeholder={placeholder} 
            
            {disabled}
            {autocapitalize}
        />
    </div>
</div>


<style>
    .text-field-container {
        margin-bottom: 16px;

        flex-direction: column;
        display: flex;
    }

    .label {
        font-size: 0.8rem;
        margin-bottom: 8px;
    }

    .text-field {
        background: rgba(225, 225, 225, 0.1);
        border-radius: 8px;

        flex-direction: column;
        display: flex;
    }


    input {
        background: none;
        border: none;
        outline: none;
        padding: 12px 8px;
        
    }

    input:disabled {
        opacity: .6;
    }

    .text-field:has(input:disabled) {
        background: rgba(225, 225, 225, 0.02);
    }
</style>