function loadPage(page) {
            fetch(page + ".html")
            .then(response => response.text())
            .then(data => {
                document.getElementById("sidebarcontainer").innerHTML = data;
            });
        }