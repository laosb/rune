import 'package:fluent_ui/fluent_ui.dart';

import '../../../utils/router/navigation.dart';
import '../../../messages/all.dart';

import '../widgets/review_connection_dialog.dart';

void showReviewConnectionDialog(
    BuildContext context, ClientSummary clientSummary) {
  $showModal<void>(
    context,
    (context, $close) => ReviewConnectionDialog(
      $close: $close,
      clientSummary: clientSummary,
    ),
    barrierDismissible: false,
    dismissWithEsc: false,
  );
}
