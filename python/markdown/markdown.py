import re


def parse(markdown):
    html = ''
    in_list = False
    in_list_append = False
    lines = markdown.split('\n')
    for line in lines:
        line = match_headers(line)
        line, in_list, in_list_append = match_bullet(
            line, in_list, in_list_append)

        line = match_paragraph(line)
        line = match_bold_and_italic(line)
        if in_list_append:
            line = '</ul>' + line
            in_list_append = False
        html += line
    if in_list:
        html += '</ul>'
    return html


def match_headers(line):
    if re.match('###### (.*)', line) is not None:
        line = '<h6>' + line[7:] + '</h6>'
    elif re.match('## (.*)', line) is not None:
        line = '<h2>' + line[3:] + '</h2>'
    elif re.match('# (.*)', line) is not None:
        line = '<h1>' + line[2:] + '</h1>'
    return line


def match_bullet(line, in_list, in_list_append):
    if bullet := re.match(r'\* (.*)', line):
        curr = bullet.group(1)
        curr = match_bold_and_italic(curr)
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


def match_bold_and_italic(curr):
    if bold := re.match('(.*)__(.*)__(.*)', curr):
        curr = bold.group(1) + '<strong>' + \
            bold.group(2) + '</strong>' + bold.group(3)

    if italic := re.match('(.*)_(.*)_(.*)', curr):
        curr = italic.group(1) + '<em>' + italic.group(2) + \
            '</em>' + italic.group(3)
    return curr


def match_paragraph(line):
    if re.match('<h|<ul|<p|<li', line) is None:
        line = '<p>' + line + '</p>'
    return line
