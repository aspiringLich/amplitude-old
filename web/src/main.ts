import hljs from 'highlight.js/lib/common';
import App from './App.svelte';

export const app = new App({
    target: document.getElementById('app'),
});

export function highlight(element) {
    element.querySelectorAll('pre>code:not(hljs)').forEach((el: HTMLElement) => {
        // count lines
        let lines = 0;
        for (let c of el.innerHTML) {
            if (c == '\n') lines++;
        }
        let line_string = '';
        for (let i = 1; i <= lines; i++) {
            line_string += i + '<br />';
        }
        // console.log(lines);
        
        let line_count = document.createElement('code');
        line_count.classList.add('line-count');
        line_count.innerHTML = line_string;
        
        let pre = el.parentElement;
        
        let container = document.createElement('div');
        container.classList.add('code-lines-container', 'hljs');
        pre.replaceChildren(container);
        
        container.appendChild(line_count);
        container.appendChild(el);
        
        hljs.highlightElement(el);
    });
}