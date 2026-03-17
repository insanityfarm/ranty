// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="intro.html">Introduction</a></li><li class="chapter-item expanded "><a href="getting-started.html"><strong aria-hidden="true">1.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="language.html"><strong aria-hidden="true">2.</strong> Language</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="language/text.html"><strong aria-hidden="true">2.1.</strong> Text</a></li><li class="chapter-item expanded "><a href="language/blocks.html"><strong aria-hidden="true">2.2.</strong> Blocks</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="language/blocks/protected-blocks.html"><strong aria-hidden="true">2.2.1.</strong> Protected blocks</a></li></ol></li><li class="chapter-item expanded "><a href="language/functions.html"><strong aria-hidden="true">2.3.</strong> Functions</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="language/functions/lambdas.html"><strong aria-hidden="true">2.3.1.</strong> Lambdas</a></li><li class="chapter-item "><a href="language/functions/optional-parameters.html"><strong aria-hidden="true">2.3.2.</strong> Optional parameters</a></li><li class="chapter-item "><a href="language/functions/variadic-parameters.html"><strong aria-hidden="true">2.3.3.</strong> Variadic parameters</a></li><li class="chapter-item "><a href="language/functions/argument-spreading.html"><strong aria-hidden="true">2.3.4.</strong> Argument spreading</a></li><li class="chapter-item "><a href="language/functions/piping.html"><strong aria-hidden="true">2.3.5.</strong> Piping</a></li></ol></li><li class="chapter-item expanded "><a href="language/comments.html"><strong aria-hidden="true">2.4.</strong> Comments</a></li><li class="chapter-item expanded "><a href="language/escape-sequences.html"><strong aria-hidden="true">2.5.</strong> Escape Sequences</a></li><li class="chapter-item expanded "><a href="language/data-types.html"><strong aria-hidden="true">2.6.</strong> Data Types</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="language/data-types/string.html"><strong aria-hidden="true">2.6.1.</strong> string</a></li><li class="chapter-item "><a href="language/data-types/int.html"><strong aria-hidden="true">2.6.2.</strong> int</a></li><li class="chapter-item "><a href="language/data-types/float.html"><strong aria-hidden="true">2.6.3.</strong> float</a></li><li class="chapter-item "><a href="language/data-types/bool.html"><strong aria-hidden="true">2.6.4.</strong> bool</a></li><li class="chapter-item "><a href="language/data-types/list.html"><strong aria-hidden="true">2.6.5.</strong> list</a></li><li class="chapter-item "><a href="language/data-types/tuple.html"><strong aria-hidden="true">2.6.6.</strong> tuple</a></li><li class="chapter-item "><a href="language/data-types/map.html"><strong aria-hidden="true">2.6.7.</strong> map</a></li><li class="chapter-item "><a href="language/data-types/range.html"><strong aria-hidden="true">2.6.8.</strong> range</a></li><li class="chapter-item "><a href="language/data-types/function.html"><strong aria-hidden="true">2.6.9.</strong> function</a></li><li class="chapter-item "><a href="language/data-types/selector.html"><strong aria-hidden="true">2.6.10.</strong> selector</a></li><li class="chapter-item "><a href="language/data-types/nothing.html"><strong aria-hidden="true">2.6.11.</strong> nothing</a></li></ol></li><li class="chapter-item expanded "><a href="language/accessors.html"><strong aria-hidden="true">2.7.</strong> Accessors</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="language/accessors/access-paths.html"><strong aria-hidden="true">2.7.1.</strong> Access Paths</a></li><li class="chapter-item "><a href="language/accessors/compound-assignment.html"><strong aria-hidden="true">2.7.2.</strong> Compound Assignment</a></li><li class="chapter-item "><a href="language/accessors/anonymous.html"><strong aria-hidden="true">2.7.3.</strong> Anonymous Accessors</a></li><li class="chapter-item "><a href="language/accessors/globals-descoping.html"><strong aria-hidden="true">2.7.4.</strong> Globals &amp; Descoping</a></li><li class="chapter-item "><a href="language/accessors/fallbacks.html"><strong aria-hidden="true">2.7.5.</strong> Fallbacks</a></li></ol></li><li class="chapter-item expanded "><a href="language/keywords.html"><strong aria-hidden="true">2.8.</strong> Keywords</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="language/keywords/require.html"><strong aria-hidden="true">2.8.1.</strong> @require</a></li><li class="chapter-item "><a href="language/keywords/return.html"><strong aria-hidden="true">2.8.2.</strong> @return</a></li><li class="chapter-item "><a href="language/keywords/continue.html"><strong aria-hidden="true">2.8.3.</strong> @continue</a></li><li class="chapter-item "><a href="language/keywords/break.html"><strong aria-hidden="true">2.8.4.</strong> @break</a></li><li class="chapter-item "><a href="language/keywords/weight.html"><strong aria-hidden="true">2.8.5.</strong> @weight</a></li><li class="chapter-item "><a href="language/keywords/text.html"><strong aria-hidden="true">2.8.6.</strong> @text</a></li></ol></li><li class="chapter-item expanded "><a href="language/operators.html"><strong aria-hidden="true">2.9.</strong> Operators</a></li><li class="chapter-item expanded "><a href="language/conditional-expressions.html"><strong aria-hidden="true">2.10.</strong> Conditional expressions</a></li><li class="chapter-item expanded "><a href="language/output-modifiers.html"><strong aria-hidden="true">2.11.</strong> Output modifiers</a></li></ol></li><li class="chapter-item expanded "><a href="runtime.html"><strong aria-hidden="true">3.</strong> Runtime Features</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="runtime/attributes.html"><strong aria-hidden="true">3.1.</strong> Attributes</a></li><li class="chapter-item expanded "><a href="runtime/formatters.html"><strong aria-hidden="true">3.2.</strong> Formatters</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="runtime/case-formatter.html"><strong aria-hidden="true">3.2.1.</strong> Case</a></li><li class="chapter-item "><a href="runtime/number-formatter.html"><strong aria-hidden="true">3.2.2.</strong> Numbers</a></li><li class="chapter-item "><a href="runtime/whitespace-formatter.html"><strong aria-hidden="true">3.2.3.</strong> Whitespace</a></li></ol></li><li class="chapter-item expanded "><a href="modules.html"><strong aria-hidden="true">3.3.</strong> Modules</a></li><li class="chapter-item expanded "><a href="cli.html"><strong aria-hidden="true">3.4.</strong> CLI / REPL</a></li></ol></li><li class="chapter-item expanded "><a href="stdlib.html"><strong aria-hidden="true">4.</strong> Standard Library</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="stdlib/general.html"><strong aria-hidden="true">4.1.</strong> General</a></li><li class="chapter-item expanded "><a href="stdlib/control-flow.html"><strong aria-hidden="true">4.2.</strong> Attributes &amp; Control Flow</a></li><li class="chapter-item expanded "><a href="stdlib/collections.html"><strong aria-hidden="true">4.3.</strong> Collections</a></li><li class="chapter-item expanded "><a href="stdlib/generators.html"><strong aria-hidden="true">4.4.</strong> Generators</a></li><li class="chapter-item expanded "><a href="stdlib/formatting.html"><strong aria-hidden="true">4.5.</strong> Formatting</a></li><li class="chapter-item expanded "><a href="stdlib/strings.html"><strong aria-hidden="true">4.6.</strong> Strings</a></li><li class="chapter-item expanded "><a href="stdlib/boolean.html"><strong aria-hidden="true">4.7.</strong> Boolean</a></li><li class="chapter-item expanded "><a href="stdlib/comparison.html"><strong aria-hidden="true">4.8.</strong> Comparison</a></li><li class="chapter-item expanded "><a href="stdlib/math.html"><strong aria-hidden="true">4.9.</strong> Math</a></li><li class="chapter-item expanded "><a href="stdlib/conversion.html"><strong aria-hidden="true">4.10.</strong> Conversion</a></li><li class="chapter-item expanded "><a href="stdlib/verification.html"><strong aria-hidden="true">4.11.</strong> Verification</a></li><li class="chapter-item expanded "><a href="stdlib/assertion.html"><strong aria-hidden="true">4.12.</strong> Assertion</a></li><li class="chapter-item expanded "><a href="stdlib/constants.html"><strong aria-hidden="true">4.13.</strong> Constants</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded "><div><strong aria-hidden="true">5.</strong> Appendix</div><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item expanded "><a href="compiler-messages.html"><strong aria-hidden="true">5.1.</strong> Diagnostics</a></li><li class="chapter-item expanded "><a href="rant-3-vs-4.html"><strong aria-hidden="true">5.2.</strong> Comparison of Rant 3 and 4</a></li><li class="chapter-item expanded "><a href="glossary.html"><strong aria-hidden="true">5.3.</strong> Glossary</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
