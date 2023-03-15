from __future__ import annotations

from pygments import highlight
from pygments.lexers import guess_lexer
from pygments.formatters import HtmlFormatter

def highlight_html(code: str)-> str:
    lexer = guess_lexer(code)
    print(lexer)
    formatter = HtmlFormatter(style='colorful')
    return highlight(code, lexer, formatter)
    return pygments.highlight(content, lexer, formatter)

