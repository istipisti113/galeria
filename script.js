function loadPage(page, id) {
  fetch(page)
    .then(response => response.text())
    .then(data => {
      document.getElementById(id).innerHTML = data;
    });
}

function add_to_cart(painting) {
  console.log(painting)
  fetch()
}
