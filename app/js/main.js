function logout() {
    Cookies.remove("token");
    location.href = "auth.html";
}

function show_write_note() {
    document.getElementById("main").innerHTML = `
    <div class="md:mx-20 lg:mx-40">
        <form
            onsubmit="write_note();return false"
            class="flex flex-col rounded border-4 border-solid border-white p-4"
        >
            <input
                id="note_title"
                minlength="1"
                maxlength="42"
                class="border-b-2 bg-black p-4 text-4xl font-semibold"
            />
            <textarea
                id="note_body"
                minlength="1"
                maxlength="420"
                class="min-h-80 mt-4 h-fit overflow-hidden bg-black p-4 text-2xl"
            ></textarea>
            <div class="mt-4 flex justify-evenly">
                <button class="rounded-2xl bg-green-600 p-2 px-4">
                    Write
                </button>
                <button
                    onclick="show_notes();return false"
                    class="rounded-2xl bg-neutral-600 p-2 px-4"
                >
                    Cancel
                </button>
            </div>
        </form>
    </div>
    `;
}

function write_note() {
    console.log("write_note");
}

async function show_notes() {
    document.getElementById("main").innerHTML = `
    <div class="md:mx-20 lg:mx-40">
        <div
            id="notes"
            class="grid justify-items-center gap-4 sm:grid-cols-2 xl:grid-cols-4"
        ></div>
    </div>
    `;

    const notes = await read_notes();

    notes.forEach((note) => {
        document.getElementById("notes").innerHTML += `
        <div
            onclick="show_note(${note.id})"
            class="h-60 w-60 cursor-pointer rounded border-4 border-solid border-white p-4"
        >
            <h class="line-clamp-1 border-b-2 text-2xl font-semibold">${note.title}</h>
            <p class="mt-4 line-clamp-6 text-base">${note.body}</p>
        </div>
        `;
    });
}

async function read_notes() {
    try {
        const response = await fetch("http://127.0.0.1:42069/api/notes", {
            method: "GET",
        });

        switch (response.status) {
            case 200:
                return await response.json();
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
    return [];
}

async function read_note(id) {
    try {
        const response = await fetch(`http://127.0.0.1:42069/api/notes/${id}`, {
            method: "GET",
        });

        switch (response.status) {
            case 200:
                return await response.json();
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
    show_notes();
}

async function show_note(id) {
    const note = await read_note(id);
    document.getElementById("main").innerHTML = `
    <div class="md:mx-20 lg:mx-40">
        <form
            onsubmit="update_note(${note.id});return false"
            class="flex flex-col rounded border-4 border-solid border-white p-4"
        >
            <input
                id="note_title"
                minlength="1"
                maxlength="42"
                class="border-b-2 bg-black p-4 text-4xl font-semibold"
                value="${note.title}"
            />
            <textarea
                id="note_body"
                minlength="1"
                maxlength="420"
                class="min-h-80 mt-4 h-fit overflow-hidden bg-black p-4 text-2xl"
            >${note.body}</textarea>
            <div class="mt-4 flex justify-evenly">
                <button class="rounded-2xl bg-blue-600 p-2 px-4">
                    Update
                </button>
                <button
                    onclick="delete_note(${note.id});return false"
                    class="rounded-2xl bg-red-600 p-2 px-4"
                >
                    Delete
                </button>
                <button
                    onclick="show_notes();return false"
                    class="rounded-2xl bg-neutral-600 p-2 px-4"
                >
                    Cancel
                </button>
            </div>
        </form>
    </div>
    `;
}

async function update_note(id) {
    const note_title = document.getElementById("note_title");
    const note_body = document.getElementById("note_body");

    const data = {
        title: note_title.value,
        body: note_body.value,
    };

    try {
        const response = await fetch(`http://127.0.0.1:42069/api/notes/${id}`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                note_title.value = response_json.title;
                note_body.value = response_json.body;
                toast("Success", "Note updated");
                break;
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

function delete_note(id) {
    console.log("delete_note");
}

// onload
(async function () {
    const token = Cookies.get("token");

    if (token == undefined) {
        location.href = "auth.html";
        return;
    }

    try {
        const response = await fetch("http://127.0.0.1:42069/api/token", {
            method: "PUT",
        });

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                Cookies.set("token", await response_json.token);
                break;
            default:
                Cookies.remove("token");
                location.href = "auth.html";
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }

    show_notes();
})();
