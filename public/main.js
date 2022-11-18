async function init() {
  let rustApp = null;
  try {
    rustApp = await import("../pkg");
  } catch (error) {
    console.error(e);
    return;
  }
  console.log(rustApp);

  const fileReader = new FileReader();
  const input = document.getElementById("upload");

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    rustApp.grayscale(base64);
  };

  input.addEventListener("change", () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
