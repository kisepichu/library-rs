echo $(pwd)

# index.md
#  ...
#  "  verificationCategories:"
# +"    - name: 'Verify Log'"
# +"      pages:"
# +"        - icon: ':heavy_check_mark:'"
# +"          path: 'verify/log'"
# +"          title: 'verify/log'"
# ...

sed -i -e '/verificationCategories:/a\    - name: "Verify Log"\n      pages:\n        - icon: ":heavy_check_mark:"\n          path: "verify/log"\n          title: "verify/log"' index.md
