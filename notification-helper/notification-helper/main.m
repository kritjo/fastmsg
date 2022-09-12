//
//  main.m
//  notification-helper
//
//    (The WTFPL)
//
//    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//    Version 2, December 2004
//
//    Copyright (C) 2013 Norio Nomura
//    Copyright (C) 2022 Kristian Tjelta Johansen <kritjo@kritjo.com>
//
//    Everyone is permitted to copy and distribute verbatim or modified
//    copies of this license document, and changing it is allowed as long
//    as the name is changed.
//
//    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//    0. You just DO WHAT THE FUCK YOU WANT TO.
//

#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>
#import <objc/runtime.h>


#pragma mark - Swizzle NSBundle

NSString *fakeBundleIdentifier = nil;
NSUUID *uuid = nil;
NSString *uuid_str = nil;

@implementation NSBundle(swizle)

// Overriding bundleIdentifier works, but overriding NSUserNotificationAlertStyle does not work.

- (NSString *)__bundleIdentifier
{
    if (self == [NSBundle mainBundle]) {
        return fakeBundleIdentifier ? fakeBundleIdentifier : @"com.apple.finder";
    } else {
        return [self __bundleIdentifier];
    }
}

@end

BOOL installNSBundleHook()
{
    Class class = objc_getClass("NSBundle");
    if (class) {
        method_exchangeImplementations(class_getInstanceMethod(class, @selector(bundleIdentifier)),
                                       class_getInstanceMethod(class, @selector(__bundleIdentifier)));
        return YES;
    }
	return NO;
}


#pragma mark - NotificationCenterDelegate

@interface NotificationCenterDelegate : NSObject<NSUserNotificationCenterDelegate>

@property (nonatomic, assign) BOOL keepRunning;

@end

@implementation NotificationCenterDelegate


- (void) userNotificationCenter:(NSUserNotificationCenter *)center didActivateNotification:(NSUserNotification *)notification{
    if (![uuid_str isEqualToString:notification.identifier]) return;
    [[NSPasteboard generalPasteboard] clearContents];
    [[NSPasteboard generalPasteboard] setString:notification.informativeText forType:NSPasteboardTypeString];
        self.keepRunning = NO;
}

@end


#pragma mark -

int main(int argc, const char * argv[])
{
    @autoreleasepool {
        if (installNSBundleHook()) {
            NSUserDefaults *defaults = [NSUserDefaults standardUserDefaults];
            
            fakeBundleIdentifier = [defaults stringForKey:@"identifier"];
            
            NSUserNotificationCenter *nc = [NSUserNotificationCenter defaultUserNotificationCenter];
            NotificationCenterDelegate *ncDelegate = [[NotificationCenterDelegate alloc]init];
            ncDelegate.keepRunning = YES;
            nc.delegate = ncDelegate;
            
            NSUserNotification *note = [[NSUserNotification alloc] init];
            note.title = [defaults stringForKey:@"title"];
            note.subtitle = [defaults stringForKey:@"subtitle"];
            note.informativeText = [defaults stringForKey:@"informativeText"];
            note.hasActionButton = true;
            note.actionButtonTitle = @"Copy to clipboard";
            // Create a uuid, because the didActivateNotification will trigger for everything. So if there are multiple running instances of the tool, we do not close all.
            uuid = [NSUUID UUID];
            uuid_str = [uuid UUIDString];
            note.identifier = uuid_str;
            
            if (!(note.title || note.subtitle || note.informativeText)) {
                note.title = @"Usage: notification-helper";
                note.informativeText = @"Options: [-identifier <IDENTIFIER>] [-title <TEXT>] [-subtitle TEXT] [-informativeText TEXT]";
            }
            
            [nc deliverNotification:note];
            
            while (ncDelegate.keepRunning) {
                [[NSRunLoop currentRunLoop] runUntilDate:[NSDate dateWithTimeIntervalSinceNow:0.1]];
            }
        }
    }
    return 0;
}
