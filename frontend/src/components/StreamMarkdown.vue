<template>
    <div v-html="renderedContent"></div>
</template>

<script setup lang="ts">
import { computed, ref, watch, onBeforeUnmount } from 'vue'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'

const props = defineProps<{
    content: string
    animate?: boolean
}>()

const md = new MarkdownIt({
    html: false,
    linkify: true,
    typographer: true,
    breaks: true,
    highlight: function (str: string, lang: string) {
        if (lang && hljs.getLanguage(lang)) {
            try {
                return '<pre class="hljs"><code>' +
                    hljs.highlight(str, { language: lang, ignoreIllegals: true }).value +
                    '</code></pre>';
            } catch {}
        }
        return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>';
    }
})

const displayedContent = ref(props.animate ? '' : props.content)

const renderedContent = computed(() => {
    return md.render(displayedContent.value)
})

let timeoutId: ReturnType<typeof setTimeout> | null = null

const typeNext = () => {
    if (!props.animate) {
        displayedContent.value = props.content
        return
    }

    if (displayedContent.value.length < props.content.length) {
        const delta = props.content.length - displayedContent.value.length
        
        let charsToAdd = 1
        if (delta > 50) charsToAdd = 5
        if (delta > 100) charsToAdd = 20
        
        displayedContent.value = props.content.slice(0, displayedContent.value.length + charsToAdd)
        
        timeoutId = setTimeout(typeNext, 15)
    } else {
        timeoutId = null
    }
}

watch(() => props.content, (newVal) => {
    if (!props.animate) {
        displayedContent.value = newVal
        return
    }
    
    if (newVal.length < displayedContent.value.length) {
        displayedContent.value = newVal
    }

    if (!timeoutId) {
        typeNext()
    }
}, { immediate: true })

watch(() => props.animate, (newVal) => {
    if (!newVal) {
        displayedContent.value = props.content
        if (timeoutId) {
            clearTimeout(timeoutId)
            timeoutId = null
        }
    }
})

onBeforeUnmount(() => {
    if (timeoutId) clearTimeout(timeoutId)
})
</script>
