function loadPage(page, id) {
  fetch(page)
    .then(response => response.text())
    .then(data => {
      document.getElementById(id).innerHTML = data;
    });
}

function add_to_cart(painting) {
  console.log(painting)
  fetch("/vetel/"+painting)
}

function search(name){
  div = document.getElementById("content")
}


function highlight(searchTerm) {
  // Remove old highlights
  document.getElementById("content").innerHTML = document.getElementById("content").innerHTML.replace(
    /<span class="highlight">(.*?)<\/span>/gi, '$1'
  );

  // Add new highlights
  if (searchTerm) {
    document.body.innerHTML = document.body.innerHTML.replace(
      new RegExp(searchTerm, 'gi'), 
      '<span class="highlight">$&</span>'
    );
  }
}

