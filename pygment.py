from __future__ import annotations

from pygments import highlight
from pygments.lexers import guess_lexer
from pygments.formatters import HtmlFormatter

def highlight_html(code: str)-> str:
    lexer = guess_lexer(code)
    print(lexer)
    formatter = HtmlFormatter(style='nord-darker')
    print(formatter.get_style_defs())
    return highlight(code, lexer, formatter)
    return pygments.highlight(content, lexer, formatter)

print(highlight_html("""
from __future__ import annotations

from pygments import highlight
from pygments.lexers import guess_lexer
from pygments.formatters import HtmlFormatter

def highlight_html(code: str)-> str:
    lexer = guess_lexer(code)
    print(lexer)
    formatter = HtmlFormatter(style='colorful')
    print(formatter.get_style_defs())
    return highlight(code, lexer, formatter)
    return pygments.highlight(content, lexer, formatter)

"""))