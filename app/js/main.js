function logout() {
    Cookies.remove("token");
    location.href = "auth";
}

function show_write_note() {
    document.getElementById("main").innerHTML = `
    <div class="max-md:mx-2 md:mx-20 lg:mx-40">
        <form
            onsubmit="write_note();return false"
            class="flex flex-col rounded border-4 border-solid border-white p-4"
        >
            <input
                id="note_title"
                type="text"
                maxlength="42"
                placeholder="Title"
                required
                class="border-b-2 bg-black p-4 text-4xl font-semibold max-md:text-2xl"
            />
            <textarea
                id="note_body"
                type="text"
                maxlength="420"
                placeholder="Body"
                required
                class="min-h-80 mt-4 h-fit bg-black p-4 text-2xl max-md:text-base"
            ></textarea>
            <div class="mt-4 flex justify-evenly max-sm:text-xs lg:text-xl">
                <button class="rounded-2xl bg-green-600 p-2 px-4">Write</button>
                <button
                    onclick="show_notes();return false"
                    class="rounded-2xl bg-orange-600 p-2 px-4"
                >
                    Cancel
                </button>
            </div>
        </form>
    </div>
    `;
}

async function write_note() {
    const note_title = document.getElementById("note_title");
    const note_body = document.getElementById("note_body");

    const data = {
        title: note_title.value.trim(),
        body: note_body.value.trim(),
    };

    if (!data.title || !data.body) {
        note_title.value = data.title;
        note_body.value = data.body;
        toast("Empty", "Empty title or body");
        return;
    }

    try {
        const response = await fetch(`http://127.0.0.1:42069/api/notes`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        switch (response.status) {
            case 201:
                toast("Success", "Note written");
                const response_json = await response.json();
                show_note(response_json.id);
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

async function show_note(id) {
    const note = await read_note(id);
    document.getElementById("main").innerHTML = `
    <div class="max-md:mx-2 md:mx-20 lg:mx-40">
        <form
            onsubmit="update_note(${note.id});return false"
            class="flex flex-col rounded border-4 border-solid border-white p-4"
        >
            <input
                id="note_title"
                type="text"
                maxlength="42"
                placeholder="Title"
                required
                class="border-b-2 bg-black p-4 text-4xl font-semibold max-md:text-2xl"
            />
            <textarea
                id="note_body"
                type="text"
                maxlength="420"
                placeholder="Body"
                required
                class="min-h-80 mt-4 h-fit bg-black p-4 text-2xl max-md:text-base"
            ></textarea>
            <div class="mt-4 flex justify-evenly max-sm:text-xs lg:text-xl">
                <button class="rounded-2xl bg-blue-600 p-2 px-4">Update</button>
                <button
                    onclick="delete_note(${note.id});return false"
                    class="rounded-2xl bg-red-600 p-2 px-4"
                >
                    Delete
                </button>
                <button
                    onclick="show_notes();return false"
                    class="rounded-2xl bg-orange-600 p-2 px-4"
                >
                    Return
                </button>
            </div>
        </form>
    </div>
    `;
    document.getElementById("note_title").value = note.title;
    document.getElementById("note_body").innerText = note.body;
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

async function show_notes() {
    document.getElementById("main").innerHTML = `
    <div class="sm:mx-20 lg:mx-32">
        <div
            id="notes"
            class="grid justify-items-center gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
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
            <h
                id="note_title_${note.id}"
                class="line-clamp-1 border-b-2 text-2xl font-semibold"
            ></h>
            <p id="note_body_${note.id}" class="mt-4 line-clamp-6 text-base"></p>
        </div>
        `;
        document.getElementById(`note_title_${note.id}`).innerText = note.title;
        document.getElementById(`note_body_${note.id}`).innerText = note.body;
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

async function update_note(id) {
    const note_title = document.getElementById("note_title");
    const note_body = document.getElementById("note_body");

    const data = {
        title: note_title.value.trim(),
        body: note_body.value.trim(),
    };

    if (!data.title || !data.body) {
        note_title.value = data.title;
        note_body.value = data.body;
        toast("Empty", "Empty title or body");
        return;
    }

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

async function delete_note(id) {
    try {
        const response = await fetch(`http://127.0.0.1:42069/api/notes/${id}`, {
            method: "DELETE",
        });

        switch (response.status) {
            case 200:
                toast("Success", "Note deleted");
                show_notes();
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

// onload
(async function () {
    const token = Cookies.get("token");

    if (token == undefined) {
        location.href = "auth";
        return;
    }

    try {
        const response = await fetch("http://127.0.0.1:42069/api/users/token", {
            method: "PUT",
        });

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                Cookies.set("token", await response_json.token);
                break;
            default:
                Cookies.remove("token");
                location.href = "auth";
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }

    show_notes();
})();
