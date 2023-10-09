window.addEventListener("DOMContentLoaded", _ => {
    const zoomableContent = document.getElementsByClassName('zoomable-content')[0];
    let scale = 1; // Initial scale factor

    // Function to handle zoom in
    document.getElementById('zoomable-div').addEventListener('wheel', (event) => {
        if (event.ctrlKey) {

            if (event.deltaY < 0) {
                event.preventDefault();
                scale += 2; // You can adjust the zoom factor as needed
                zoomableContent.style.transform = `scale(${scale})`;
            } else if (event.deltaY > 0) {
                if (scale > 1) {
                    scale -= 2; // You can adjust the zoom factor as needed
                    zoomableContent.style.transform = `scale(${scale})`;
                }
            }
        }
    });

    document.getElementById('download-png').addEventListener('click', () => {
        const canvas = document.getElementById("canvas");

        // Convert the Canvas to a data URL
        const dataURL = canvas.toDataURL("image/png");

        // Create a link element to trigger the download
        const downloadLink = document.createElement("a");
        downloadLink.href = dataURL;
        downloadLink.download = `game_of_life_${Date.now()}.png`;

        // Simulate a click to trigger the download
        downloadLink.click();
    });


    document.getElementById('restart').addEventListener('click', () => {
        window.location.reload();
    });

    const playButton = document.getElementById('play');

    playButton.addEventListener('click', () => {
        if (playButton.value === "stop") {
            playButton.textContent = "STOP";
            playButton.value = "start";
        } else {
            playButton.textContent = "START";
            playButton.value = "stop";
        }
    });
})
