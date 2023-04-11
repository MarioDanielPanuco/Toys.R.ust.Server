async function submitContactForm() {
    const senderEmail = document.getElementById("sender_email").value;
    const subjectTitle = document.getElementById("subject_title").value;
    const messageBody = document.getElementById("message_body").value;

    const formData = {
        sender_email: senderEmail,
        subject_title: subjectTitle,
        message_body: messageBody
    };

    const response = await fetch("/contact_form", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(formData)
    });

    alert(response.status);
    if (response.ok) {
        alert("Email sent successfully!")
    } else {
        alert("Error sending email.");
    }
}

async function react_component () {
    //TODO: react comp
}
