function logout() {
    Cookies.remove("token");
    location.href = "auth.html";
}
