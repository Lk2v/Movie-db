<script lang="ts">
	import type { MovieTag } from "$api/movie";
	import { timeAgo } from "$api/time";
	import IconButton from "$components/IconButton.svelte";
	import { Trash, UserMinus } from "lucide-svelte";

    let {
        index,
        tag,
        onRemoveTag,
        onRemoveUser
    }: {
        index: number,
        tag: MovieTag;
        onRemoveTag: () => void;
        onRemoveUser: () => void;
    }= $props();
</script>

<div class="tag-container" style="--delay: {index * 200}ms;">
    <div class="tag-body">
        <div class="tag-content">
            <h2 class="tag-name">
                <span class="quote-mark left">“</span>
                {tag.tag}
                <span class="quote-mark right">”</span>
            </h2>
        </div>
        <h5 class="tag-timestamp">{timeAgo(tag.timestamp)}</h5>
    </div>
    <div class="tag-footer">
        <IconButton onclick={onRemoveTag}>
            <Trash size={16}/>
        </IconButton>
    
        <IconButton onclick={onRemoveUser}>
            <UserMinus size={16}/>
        </IconButton>
    </div>
</div>

<style>
    .tag-container {
        margin: 1rem 0;
        flex-direction: column;
        display: flex;

        animation-name: fade;
        animation-duration: 1s;
        animation-fill-mode: backwards;
        animation-delay: var(--delay);
    }

    @keyframes fade {
        from {
            opacity: 0;
            transform: scale(0);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }
    }

    .tag-body {

    }

    .tag-footer {
        flex-direction: row;
        display: flex;
    }

    .tag-name {
        margin-bottom: 0.5rem;
    }

    .quote-mark {
        opacity: 0.5;
    }

    .tag-timestamp {
        margin: 0;
        font-weight: bold;
        opacity: 0.5;
    }

    .tag-timestamp:before {
        content: "—";
        margin-right: 0.5rem;
    }
</style>