const worker = new Worker("./worker.js", {type:"module"});

worker.addEventListener("message", async ({ data }) => {
    console.log(data);
});

worker.onerror = function (event) {
    throw new Error("s" + event.message + " (" + event.filename + ":" + event.lineno + ")");
}

window.sendMessage = function (args) {
    console.log("sent message 22")
    worker.postMessage(args);
};


async function onclick() {
    var files = document.getElementById('file').files;
    var file = files[0];
    var url = URL.createObjectURL(file)
    sendMessage(url)
}

document.getElementById("btn").addEventListener("click", onclick, false);