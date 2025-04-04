// Checks if a user was setup, if not, redirect to the setup page.
// If a user is not logged in, redirect him to the login page
export default defineNuxtRouteMiddleware(async (to, from) => {
	const origin = from.path;
	const destination = to.path;
	const to_login = destination === "/user/login";
	const to_setup = destination === "/setup";
	const to_error = destination === "/error";

	// Don't do anything on error
	if (to_error) {
		return;
	}

	// Request the user count
	const api_url = useRuntimeConfig().public.api_url;
	const url = `${api_url}/api/stats/user_count`;
	const options = {
		method: "GET",
	};
	const request = await apiRequest(url, options);
	// If there is an error during the request, send the user to the error page
	if (
		request.is_request_error ||
		request.is_response_error ||
		request.data.is_error
	) {
		console.error("Could not parse the user count reqest.");
		return navigateTo("/error");
	}
	const user_count = request.data.data.count;

	// If there are no registered Users, send the user to the register page
	if (user_count <= 0) {
		if (!to_setup) {
			console.log("Sending user to setup page");
			return navigateTo("/setup");
		}

		// Exit the function early if the user is going to the setup page, because session validity is not required.
		return;
	}

	// if there is at least one registered user, forbid the setup page
	if (user_count > 0 && to_setup) {
		// Send the user back where he came from or to the main route if the origin cannot be resolved
		if (origin === destination) {
			return navigateTo("/");
		}

		return navigateTo(origin);
	}

	// Check the validity of the session
	const valid_session = await verifySession();

	// if the user does not have a valid session, send him to the login page
	if (!valid_session && !to_login) {
		return navigateTo("/user/login");
	}

	// Disallow the login route for already logged in users
	if (valid_session && to_login) {
		if (origin === destination) {
			return navigateTo("/");
		}

		return navigateTo(origin);
	}
});
