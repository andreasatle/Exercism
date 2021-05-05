'''Refactor a Markdown parser.

The markdown exercise is a refactoring exercise.
There is code that parses a given string with Markdown syntax 
and returns the associated HTML for that string. Even though 
this code is confusingly written and hard to follow, somehow
it works and all the tests are passing! Your challenge is to
re-write this code to make it easier to read and maintain while
still making sure that all the tests keep passing.'''

import re


def parse(markdown: str) -> str:
    '''Markdown to html parser.'''
    html = ''
    in_list = False
    in_list_append = False
    lines = markdown.split('\n')
    for line in lines:
        line = _match_headers(line)
        line, in_list, in_list_append = _match_bullet(
            line, in_list, in_list_append)

        line = _match_paragraph(line)
        line = _match_bold_and_italic(line)
        if in_list_append:
            line = '</ul>' + line
            in_list_append = False
        html += line
    if in_list:
        html += '</ul>'
    return html


def _match_headers(line):
    if re.match('###### (.*)', line) is not None:
        line = '<h6>' + line[7:] + '</h6>'
    elif re.match('## (.*)', line) is not None:
        line = '<h2>' + line[3:] + '</h2>'
    elif re.match('# (.*)', line) is not None:
        line = '<h1>' + line[2:] + '</h1>'
    return line


def _match_bullet(line, in_list, in_list_append):
    if bullet := re.match(r'\* (.*)', line):
        curr = bullet.group(1)
        curr = _match_bold_and_italic(curr)
        if not in_list:
            in_list = True
            line = '<ul><li>' + curr + '</li>'
        else:
            line = '<li>' + curr + '</li>'
    else:
        if in_list:
            in_list_append = True
            in_list = False
    return line, in_list, in_list_append


def _match_bold_and_italic(curr):
    if bold := re.match('(.*)__(.*)__(.*)', curr):
        curr = bold.group(1) + '<strong>' + \
            bold.group(2) + '</strong>' + bold.group(3)

    if italic := re.match('(.*)_(.*)_(.*)', curr):
        curr = italic.group(1) + '<em>' + italic.group(2) + \
            '</em>' + italic.group(3)
    return curr


def _match_paragraph(line):
    if re.match('<h|<ul|<p|<li', line) is None:
        line = '<p>' + line + '</p>'
    return line
