#!/usr/bin/env python3
"""Check a running documentation preview using Python's standard library.

Usage: python3 checks/check_navigation.py http://127.0.0.1:18080/guide/master/
"""
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from html.parser import HTMLParser
from urllib.parse import unquote, urldefrag, urljoin, urlparse
from urllib.request import urlopen
import sys
import re
from pathlib import Path


class Page(HTMLParser):
    def __init__(self, html):
        super().__init__()
        self.links = []
        self.ids = []
        self.text = []
        self.feed(html)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if "id" in attrs:
            self.ids.append(attrs["id"])
        if tag == "a" and "href" in attrs:
            self.links.append(attrs["href"])

    def handle_data(self, data):
        self.text.append(data)


def fetch(url):
    with urlopen(url, timeout=30) as response:
        return response.geturl(), Page(response.read().decode())


def main():
    base = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:18080/guide/master/"
    canonical, home = fetch(base)
    assert "Cot documentation" in " ".join(home.text), "Expected the new documentation landing page"
    # Section-scoped sidebars intentionally do not expose every page on home.
    # Seed from the registry so even an accidentally orphaned page is checked.
    registry = Path(__file__).resolve().parents[1] / "src" / "navigation.rs"
    page_names = re.findall(r'md_page!\("([^"\n]+)"\)', registry.read_text())
    urls = sorted({base, *(urljoin(base, name + "/") for name in page_names)})
    failures = []
    pages = {}
    with ThreadPoolExecutor(max_workers=6) as pool:
        for url, result in zip(urls, pool.map(fetch, urls)):
            pages[url] = result
    checked = 0
    for url, (actual, page) in pages.items():
        duplicates = [key for key, count in Counter(page.ids).items() if count > 1]
        if duplicates:
            failures.append(f"{url}: duplicate IDs: {duplicates}")
        for link in page.links:
            dest, fragment = urldefrag(urljoin(actual, link))
            if not dest.startswith(base):
                continue
            if dest not in pages:
                try:
                    pages_result = fetch(dest)
                except Exception as error:
                    failures.append(f"{url} -> {dest}: {error}")
                    continue
            else:
                pages_result = pages[dest]
            if fragment and unquote(fragment) not in pages_result[1].ids:
                failures.append(f"{url} -> {dest}#{fragment}: missing anchor")
            checked += 1
    if failures:
        print("\n".join(sorted(set(failures))))
        raise SystemExit(1)
    print(f"Checked {len(pages)} registered page URLs and {checked} internal links; no duplicate IDs or missing anchors.")


if __name__ == "__main__":
    main()
