# Chapter-Extractor
A cli utility to extract chapters of books in pdf format.

# Why does it exist
I created it because I wanted to automate the process of creating chapter
summaries for novels using LLM.

# How to use
```chapters <path to file> <output directory>```

# Limitations
- Currently it only uses the top level bookmarks ignoring sub-bookmarks.
- Single threaded. It uses mupdf c bindidngs. So mupdf can't be shared across
threads safely. (It was able to extract all the pages from the count of monte
cristo in less than two seconds)
- Better error handling
